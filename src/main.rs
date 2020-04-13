use chrono::{DateTime, Utc, Duration};
use serde_derive::{Deserialize, Serialize};
use iced::{
    Application, Command, Element, Settings, Subscription,
    executor, scrollable, text_input, button,
    Length, HorizontalAlignment, Align, Color,
    Text, TextInput, Container, Scrollable, Column, Row, Button, Space, Checkbox,
};
use iced_native::{Event, input::{self, keyboard}};


fn main() {
    MainUi::run(Settings::default())
}


pub mod style {
    pub const SECTION_GAP: u16 = 30;
    pub const ITEM_GAP: u16 = 12;
    pub const LIST_GAP: u16 = 5;
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum HowItWasStopped {
    SelfCrashed,
    ManuallyKilled,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Record {
    #[serde(
        serialize_with="opt_dt_serde::serialize",
        deserialize_with="opt_dt_serde::deserialize")]
    frozen: Option<DateTime<Utc>>,
    #[serde(
        serialize_with="opt_dt_serde::serialize",
        deserialize_with="opt_dt_serde::deserialize")]
    busy: Option<DateTime<Utc>>,
    description: String,
    how: HowItWasStopped,
    #[serde(
        serialize_with="dt_serde::serialize",
        deserialize_with="dt_serde::deserialize")]
    when: DateTime<Utc>,
}


impl Default for Record {
    fn default() -> Self {
        Self {
            frozen: None,
            busy: None,
            description: String::new(),
            how: HowItWasStopped::SelfCrashed,
            when: Utc::now(),
        }
    }
}

macro_rules! UiElement {
    () => { Element<<MainUi as Application>::Message> };
    (for<$lf:lifetime>) => { Element<$lf, <MainUi as Application>::Message> };
}

macro_rules! UiMessage {
    () => { <MainUi as Application>::Message };
}

#[derive(Default)]
struct MainUi {
    data: Option<MainData>,
    ui: MainUiState,
}


#[derive(Debug, Clone)]
enum LoadError {
    FileError,
    FormatError,
}


#[derive(Debug, Clone)]
enum SaveError {
    DirectoryError,
    FileError,
    WriteError,
    FormatError,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct MainData {
    records: Vec<Record>,
}


#[derive(Default)]
struct MainUiState {
    layout_debug: bool,
    last_tick: Option<DateTime<Utc>>,
    edit: MainUiEditState,
    // layout
    records_scroll_state: scrollable::State,
}


#[derive(Default, Clone)]
struct MainUiEditState {
    record: Record,
    // widgets
    description_state: text_input::State,
    crash_5sec_state: button::State,
    killed_state: button::State,
    clear_state: button::State,
}


impl MainUi {
    fn register_entry(&mut self) -> Command<UiMessage!()>
    {
        if let Some(ref mut data) = &mut self.data.as_mut() {
            let edit = std::mem::take(&mut self.ui.edit);
            data.records.push(edit.record);
        };

        self.save_command()
    }

    fn save_command(&mut self) -> Command<UiMessage!()>
    {
        match &mut self.data {
            Some(ref mut data) => Command::perform(data.clone().save(), Message::Saved),
            None => Command::none()
        }
    }
}



#[derive(Debug, Clone)]
enum Message {
    DataLoaded(Result<MainData, LoadError>),
    Saved(Result<(), SaveError>),
    Tick(DateTime<Utc>),
    EventOccurred(Event),
    DescriptionEdited(String),
    FrozenToggled(bool),
    BusyToggled(bool),
    Crash5secClicked,
    KilledClicked,
    ClearClicked,
}


impl Application for MainUi {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self::default(),
            Command::perform(MainData::load(), Message::DataLoaded)
        )
    }

    fn title(&self) -> String {
        "Crash recorder".into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            time::every(std::time::Duration::from_millis(1000)).map(Message::Tick),
            iced_native::subscription::events().map(Message::EventOccurred),
        ])
    }

    fn update(&mut self, msg: Message) -> Command<Self::Message> {
        match msg {
            Message::DataLoaded(Ok(data)) => {
                self.data = Some(data);
            },
            Message::DataLoaded(Err(_)) => {
                self.data = Some(MainData::default());
            },
            Message::Saved(_) => (),
            Message::Tick(when) => {
                self.ui.last_tick = Some(when);
            },
            Message::EventOccurred(event) => {
                match event {
                    Event::Keyboard(keyboard::Event::Input{
                        state: input::ButtonState::Pressed,
                        key_code: keyboard::KeyCode::F11,
                        modifiers: keyboard::ModifiersState {
                            shift: false, control: false, alt: false, logo: false,
                        }
                    }) => {
                        self.ui.layout_debug = !self.ui.layout_debug;
                    },
                    _ => ()
                }
            },
            Message::DescriptionEdited(value) => {
                self.ui.edit.record.description = value;
            },
            Message::FrozenToggled(checked) => {
                self.ui.edit.record.frozen =
                    if checked { Some(Utc::now()) }
                    else { None };
            },
            Message::BusyToggled(checked) => {
                self.ui.edit.record.busy =
                    if checked { Some(Utc::now()) }
                    else { None };
            },
            Message::Crash5secClicked => {
                self.ui.edit.record.how = HowItWasStopped::SelfCrashed;
                self.ui.edit.record.when = Utc::now();
                return self.register_entry();
            },
            Message::KilledClicked => {
                self.ui.edit.record.how = HowItWasStopped::ManuallyKilled;
                self.ui.edit.record.when = Utc::now();
                return self.register_entry();
            },
            Message::ClearClicked => {
                if let Some(ref mut data) = &mut self.data {
                    *data = MainData::default();
                };
                return self.save_command();
            },
        }

        Command::none()
    }

    fn view(&mut self) -> UiElement!() {
        let now = self.ui.last_tick.unwrap_or_else(Utc::now);
        let duration_format = |d: Duration| format!(
            "{:02}:{:02}:{:02}",
            d.num_hours(),
            d.num_minutes(),
            d.num_seconds());

        let frozen_spent = match self.ui.edit.record.frozen {
            Some(when) => format!(" {} ago", duration_format(now - when)),
            None => String::new()
        };

        let busy_spent = match self.ui.edit.record.busy {
            Some(when) => format!(" {} ago", duration_format(now - when)),
            None => String::new()
        };

        //
        let mut rows = vec![
                Self::make_title("VS Crash report"),
                Self::make_vspace(style::SECTION_GAP),
                Self::make_label("New report"),
                Self::make_vspace(style::ITEM_GAP)];

        rows.push(TextInput::new(
            &mut self.ui.edit.description_state,
            "Description...",
            &self.ui.edit.record.description,
            Message::DescriptionEdited)
            .size(16) // font size
            .padding(5)
            .width(Length::Fill)
            .into());

        rows.append(&mut vec![
            Self::make_vspace(style::ITEM_GAP),
            Self::make_row(vec![
                Self::make_checkbox(self.ui.edit.record.frozen.is_some(),
                                    "Frozen",
                                    Message::FrozenToggled),
                Self::make_hspace(style::ITEM_GAP),
                Self::make_label(frozen_spent),
            ]),
            Self::make_vspace(style::ITEM_GAP),
            Self::make_row(vec![
                Self::make_checkbox(self.ui.edit.record.busy.is_some(),
                                    "Busy",
                                    Message::BusyToggled),
                Self::make_hspace(style::ITEM_GAP),
                Self::make_label(busy_spent),
            ]),
            Self::make_vspace(style::ITEM_GAP),
            Self::make_row(vec![
                Self::make_button(&mut self.ui.edit.crash_5sec_state,
                                  "5s ago",
                                  Message::Crash5secClicked),
                Self::make_hspace(style::ITEM_GAP),
                Self::make_button(&mut self.ui.edit.killed_state,
                                  "Killed",
                                  Message::KilledClicked),
            ]),
            Self::make_vspace(style::SECTION_GAP),
            Self::make_label(format!("History ({})",
                                     self.data.as_ref().map_or(
                                         0,
                                         |data| data.records.len()))),
            Self::make_vspace(style::ITEM_GAP),
        ]);

        let items = match &self.data {
            None => {
                Self::make_placeholder("No records.")
            },
            Some(ref data) if data.records.is_empty() => {
                Self::make_placeholder("No records.")
            },
            Some(ref data) => {
                Column::with_children(data.records.iter().map(Self::make_entry).collect())
                    .width(Length::Fill)
                    .spacing(style::LIST_GAP)
                    .into()
            }
        };

        rows.push(Scrollable::new(&mut self.ui.records_scroll_state)
                  .width(Length::Fill)
                  .padding(5)
                  .push(items)
                  .into());

        if self.data.as_ref().map(|data| !data.records.is_empty()).unwrap_or(false) {
            rows.append(
                &mut vec![
                Self::make_vspace(style::ITEM_GAP),
                Self::make_button(
                    &mut self.ui.edit.clear_state,
                    "Clear",
                    Message::ClearClicked)
                ]);
        };

        Self::make_root(self.ui.layout_debug, rows)
    }
}

impl MainUi {
    fn make_root<'a>(debug: bool,
                     items: Vec<UiElement!(for<'a>)>) -> UiElement!(for<'a>) {
        let mut central: UiElement!() =
            Column::with_children(items)
            .padding(20)
            .width(Length::Units(500)) // window content width
            .into();

        if debug {
            central = central.explain(Color::BLACK)
        }

        Container::new(central)
            .width(Length::Fill) // fill the window width
            .center_x()
            .into()
    }

    fn make_row(items: Vec<UiElement!()>) -> UiElement!() {
        Row::with_children(items)
            .width(Length::Fill)
            .align_items(Align::Center)
            .into()
    }

    fn make_title(title: &str) -> UiElement!() {
        Text::new(title)
            .horizontal_alignment(HorizontalAlignment::Center)
            .width(Length::Fill)
            .size(22)
            .color([0.1, 0.1, 0.1])
            .into()
    }

    fn make_placeholder(text: &str) -> UiElement!() {
        Text::new(text)
            .horizontal_alignment(HorizontalAlignment::Center)
            .width(Length::Fill)
            .size(18)
            .color([0.1, 0.1, 0.3])
            .into()
    }

    fn make_label<T>(label: T) -> UiElement!(for<'static>)
        where T: Into<String>
    {
        Text::new(label)
            .size(18) // font size
            .color([0.2, 0.2, 0.2]) // font color
            .into()
    }

    fn make_button<'a>(state: &'a mut button::State, text: &str,
                   msg: UiMessage!()) -> UiElement!(for<'a>) {
        Button::new(state, Text::new(text))
            .min_width(50)
            .min_height(20)
            .padding(10)
            .on_press(msg)
            .into()
    }

    fn make_checkbox(state: bool, text: &str, msg: fn(bool) -> UiMessage!()) -> UiElement!() {
        Checkbox::new(state, text, msg)
            .spacing(8)
            .text_size(18)
            .into()
    }

    fn make_vspace(space: u16) -> UiElement!(for<'static>) {
        Space::new(Length::Shrink, Length::Units(space)).into()
    }

    fn make_hspace(space: u16) -> UiElement!(for<'static>) {
        Space::new(Length::Units(space), Length::Shrink).into()
    }

    fn make_entry<'a, 'b>(entry: &'a Record) -> UiElement!(for<'b>) {
        //let dt_format = |d: DateTime<_>| d.format("%Y-%m-%d %H:%M:%S");
        let time_format = |d: DateTime<_>| d.format("%H:%M:%S");

        let mut text = String::new();

        if let Some(when) = entry.frozen {
            text.push_str(&format!("Frozen from {}", time_format(when)));
        }

        if let Some(when) = entry.busy {
            if text.is_empty() {
                text = format!("Busy from {}", time_format(when));
            }
            else {
                text.push_str(&format!(", then busy at {}", time_format(when)));
            }
        }

        if text.is_empty() {
            let when = time_format(entry.when);
            text = match entry.how {
                HowItWasStopped::SelfCrashed => format!("Crashed at {}", when),
                HowItWasStopped::ManuallyKilled  => format!("Killed at {}", when),
            };
        }
        else {
            let when = time_format(entry.when);
            text.push_str(&match entry.how {
                HowItWasStopped::SelfCrashed => format!(", and crashed at {}", when),
                HowItWasStopped::ManuallyKilled => format!(", and killed at {}", when),
            });
        }

        if !entry.description.is_empty() {
            text.push_str(&format!(" ({})", entry.description));
        }

        Self::make_label(text)
    }

}

// modified from 'iced/example/todos'
impl MainData {
    fn path() -> std::path::PathBuf {
        let mut path = match directories::ProjectDirs::from("rs", "evolix1", "Crash Recorder") {
            Some(project_dirs) => project_dirs.data_dir().into(),
            None => std::env::current_dir().unwrap_or(std::path::PathBuf::new())
        };

        path.push("records.json");

        path
    }

    async fn load() -> Result<MainData, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        serde_json::from_str(&contents)
            .map_err(|_| LoadError::FormatError)
    }

    async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&self)
            .map_err(|_| SaveError::FormatError)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::DirectoryError)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::FileError)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }

        // This is a simple way to save at most once every couple seconds
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}

// modified from [https://earvinkayonga.com/posts/deserialize-date-in-rust/]
pub mod dt_serde {
    use chrono::{DateTime, Utc};
    use serde::*;

    pub fn serialize<S: Serializer>(dt: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error> {
        dt.to_rfc3339()
            .serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<DateTime<Utc>, D::Error> {
        let time: String = Deserialize::deserialize(d)?;
        DateTime::parse_from_rfc3339(&time)
            .map_err(serde::de::Error::custom)
            .map(|fixed_dt| fixed_dt.into())
    }
}

pub mod opt_dt_serde {
    use chrono::{DateTime, Utc};
    use serde::*;

    pub fn serialize<S: Serializer>(dt: &Option<DateTime<Utc>>, s: S) -> Result<S::Ok, S::Error> {
        dt.as_ref()
            .map(|dt| dt.to_rfc3339())
            .serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<DateTime<Utc>>, D::Error> {
        let time: String = match Deserialize::deserialize(d) {
            Ok(v) => v,
            // erase error from reading, bc it can happen with `null` value
            Err(_) => return Ok(None),
        };
        DateTime::parse_from_rfc3339(&time)
            .map_err(serde::de::Error::custom)
            .map(|fixed_dt| Some(fixed_dt.into()))
    }
}

// taken from 'iced/example/clock'
mod time {
    use iced::futures;

    pub fn every(
        duration: std::time::Duration,
    ) -> iced::Subscription<chrono::DateTime<chrono::Utc>> {
        iced::Subscription::from_recipe(Every(duration))
    }

    struct Every(std::time::Duration);

    impl<H, I> iced_native::subscription::Recipe<H, I> for Every
    where
        H: std::hash::Hasher,
    {
        type Output = chrono::DateTime<chrono::Utc>;

        fn hash(&self, state: &mut H) {
            use std::hash::Hash;

            std::any::TypeId::of::<Self>().hash(state);
            self.0.hash(state);
        }

        fn stream(
            self: Box<Self>,
            _input: futures::stream::BoxStream<'static, I>,
        ) -> futures::stream::BoxStream<'static, Self::Output> {
            use futures::stream::StreamExt;

            async_std::stream::interval(self.0)
                .map(|_| chrono::Utc::now())
                .boxed()
        }
    }
}
