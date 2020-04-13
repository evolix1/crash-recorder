use chrono::{DateTime, Utc, Duration};
#[allow(unused_imports)]
use iced::{
    Application, Command, Subscription,
    executor, scrollable, text_input, button,
    Length, HorizontalAlignment, Align, Color,
    Text, TextInput, Container, Scrollable, Column, Row, Button, Space, Checkbox,
};
use iced_native::{Event, input::{self, keyboard}};

use crate::record::{Record, HowItWasStopped};
use crate::app_data::{AppData, LoadError, SaveError};

use super::utils::{time_utils};
use super::style::{style};
use super::builder::{UiBuilder};


#[derive(Default, Clone)]
struct AppUiEditState {
    record: Record,
    // widgets
    description_state: text_input::State,
    crash_5sec_state: button::State,
    killed_state: button::State,
    clear_state: button::State,
}


#[derive(Default)]
struct AppUiState {
    layout_debug: bool,
    last_tick: Option<DateTime<Utc>>,
    edit: AppUiEditState,
    // layout
    records_scroll_state: scrollable::State,
}


#[derive(Default)]
pub struct AppUi {
    data: Option<AppData>,
    ui: AppUiState,
}


#[derive(Debug, Clone)]
pub enum Message {
    DataLoaded(Result<AppData, LoadError>),
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


impl Application for AppUi {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self::default(),
            Command::perform(AppData::load(), Message::DataLoaded)
        )
    }

    fn title(&self) -> String {
        "Crash recorder".into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            time_utils::every(std::time::Duration::from_millis(1000)).map(Message::Tick),
            iced_native::subscription::events().map(Message::EventOccurred),
        ])
    }

    fn update(&mut self, msg: Message) -> Command<Self::Message> {
        match msg {
            Message::DataLoaded(Ok(data)) => {
                self.data = Some(data);
            },
            Message::DataLoaded(Err(_)) => {
                self.data = Some(AppData::default());
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
                    *data = AppData::default();
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
                UiBuilder::make_title("VS Crash report"),
                UiBuilder::make_vspace(style::SECTION_GAP),
                UiBuilder::make_label("New report"),
                UiBuilder::make_vspace(style::ITEM_GAP)];

        rows.push(TextInput::new(
            &mut self.ui.edit.description_state,
            "Description...",
            &self.ui.edit.record.description,
            Message::DescriptionEdited)
            .size(16) // font size
            .padding(5)
            .width(Length::Fill)
            .into());

        let crash_5sec_button = UiBuilder::make_button(
            &mut self.ui.edit.crash_5sec_state,
            "5s ago",
            Message::Crash5secClicked);

        let killed_button = UiBuilder::make_button(
            &mut self.ui.edit.killed_state,
            "Killed",
            Message::KilledClicked);

        rows.append(&mut vec![
            UiBuilder::make_vspace(style::ITEM_GAP),
            UiBuilder::make_form_row(
                UiBuilder::make_checkbox(self.ui.edit.record.frozen.is_some(),
                                    "Frozen",
                                    Message::FrozenToggled),
                UiBuilder::make_label(frozen_spent),
            ),
            UiBuilder::make_vspace(style::ITEM_GAP),
            UiBuilder::make_form_row(
                UiBuilder::make_checkbox(self.ui.edit.record.busy.is_some(),
                                    "Busy",
                                    Message::BusyToggled),
                UiBuilder::make_label(busy_spent),
            ),
            UiBuilder::make_vspace(style::ITEM_GAP),
            UiBuilder::make_button_row(
                /* left */ vec![],
                /* right */ vec![crash_5sec_button, killed_button]),
            UiBuilder::make_vspace(style::SECTION_GAP),

        ]);

        let mut history_row = vec![
            UiBuilder::make_label(format!("History ({})",
                self.data.as_ref().map_or(
                    0,
                    |data| data.records.len()))),
        ];

        if self.data.as_ref().map(|data| !data.records.is_empty()).unwrap_or(false) {
            history_row.append(
                &mut vec![
                UiBuilder::make_hfiller(),
                UiBuilder::make_button(
                    &mut self.ui.edit.clear_state,
                    "Clear",
                    Message::ClearClicked)
                ]);
        };

        rows.append(&mut vec![
            UiBuilder::make_row(history_row),
            UiBuilder::make_vspace(style::ITEM_GAP),
        ]);

        let record_row = match &self.data {
            None => {
                UiBuilder::make_placeholder("No records.")
            },
            Some(ref data) if data.records.is_empty() => {
                UiBuilder::make_placeholder("No records.")
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
                  .push(record_row)
                  .into());

        UiBuilder::make_root(self.ui.layout_debug, rows)
    }
}

impl AppUi {

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

        UiBuilder::make_label(text)
    }

}
