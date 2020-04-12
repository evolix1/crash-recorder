use chrono::{DateTime, Utc, Duration};

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


enum StoppedReason {
    SelfCrashed,
    ManuallyKilled,
}


struct Record {
    frozen: Option<DateTime<Utc>>,
    busy: Option<DateTime<Utc>>,
    description: String,
    stopped: StoppedReason,
    when_stopped: DateTime<Utc>
}


impl Default for Record {
    fn default() -> Self {
        Self {
            frozen: None,
            busy: None,
            description: String::new(),
            stopped: StoppedReason::SelfCrashed,
            when_stopped: Utc::now(),
        }
    }
}


#[derive(Default)]
struct MainUi {
    records: Vec<Record>,
    ui: MainUiState,
}


#[derive(Default)]
struct MainUiState {
    layout_debug: bool,
    last_tick: Option<DateTime<Utc>>,
    edit: MainUiEditState,
    // layout
    app_scroll_state: scrollable::State,
    _records_scroll_state: scrollable::State,
}


#[derive(Default)]
struct MainUiEditState {
    record: Record,
    // widgets
    description_state: text_input::State,
    crash_5sec_state: button::State,
    killed_state: button::State,
}


impl MainUi {
    fn register_entry(&mut self)
    {
        let edit = std::mem::take(&mut self.ui.edit);
        self.records.push(edit.record);
    }
}



#[derive(Debug, Clone)]
enum Message {
    Tick(DateTime<Utc>),
    EventOccurred(Event),
    DescriptionEdited(String),
    FrozenToggled(bool),
    BusyToggled(bool),
    Crash5secClicked,
    KilledClicked,
}

macro_rules! UiElement {
    () => { Element<<MainUi as Application>::Message> };
    (for<$lf:lifetime>) => { Element<$lf, <MainUi as Application>::Message> };
}

macro_rules! UiMessage {
    () => { <MainUi as Application>::Message };
}


impl Application for MainUi {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Crash reporter".into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            time::every(std::time::Duration::from_millis(1000)).map(Message::Tick),
            iced_native::subscription::events().map(Message::EventOccurred),
        ])
    }

    fn update(&mut self, msg: Message) -> Command<Self::Message> {
        match msg {
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
                self.ui.edit.record.stopped = StoppedReason::SelfCrashed;
                self.ui.edit.record.when_stopped = Utc::now();
                self.register_entry();
            },
            Message::KilledClicked => {
                self.ui.edit.record.stopped = StoppedReason::ManuallyKilled;
                self.ui.edit.record.when_stopped = Utc::now();
                self.register_entry();
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

        let description_input = TextInput::new(
            &mut self.ui.edit.description_state,
            "Description...",
            &self.ui.edit.record.description,
            Message::DescriptionEdited)
            .size(16) // font size
            .padding(5)
            .width(Length::Fill);

        let frozen_spent = match self.ui.edit.record.frozen {
            Some(when) => format!(" {} ago", duration_format(now - when)),
            None => String::new()
        };

        let busy_spent = match self.ui.edit.record.busy {
            Some(when) => format!(" {} ago", duration_format(now - when)),
            None => String::new()
        };

        let lines = vec![
                Self::make_title("VS Crash report"),
                Self::make_vspace(style::SECTION_GAP),
                Self::make_label("New report"),
                Self::make_vspace(style::ITEM_GAP),
                description_input.into(),
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
                Self::make_label(format!("History ({})", self.records.len())),
                Self::make_vspace(style::ITEM_GAP),
                Column::with_children(self.records.iter()
                                      .map(Self::make_entry)
                                      .collect())
                    .spacing(style::LIST_GAP)
                    .into()
            ];

        Self::make_root(
            &mut self.ui.app_scroll_state,
            self.ui.layout_debug,
            lines)
    }
}

impl MainUi {
    fn make_root<'a>(state: &'a mut scrollable::State,
                     debug: bool,
                     items: Vec<UiElement!(for<'a>)>) -> UiElement!(for<'a>) {
        let mut central: UiElement!() = Scrollable::new(state)
            .push(Column::with_children(items)
                  .padding(20)
                  .width(Length::Units(500))) // window content width
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

    fn make_entry<'a>(entry: &'a Record) -> UiElement!(for<'a>) {
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
            let when = time_format(entry.when_stopped);
            text = match entry.stopped {
                StoppedReason::SelfCrashed => format!("Crashed at {}", when),
                StoppedReason::ManuallyKilled  => format!("Killed at {}", when),
            };
        }
        else {
            let when = time_format(entry.when_stopped);
            text.push_str(&match entry.stopped {
                StoppedReason::SelfCrashed => format!(", and crashed at {}", when),
                StoppedReason::ManuallyKilled  => format!(", and killed at {}", when),
            });
        }

        if !entry.description.is_empty() {
            text.push_str(&format!(" ({})", entry.description));
        }

        Self::make_label(text)
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
