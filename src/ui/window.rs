use chrono::{DateTime, Utc, Duration};
use iced::{
    Application, Command, Subscription,
    executor, text_input, button, scrollable,
};
use iced_native::{Event, input::{self, keyboard}};

use crate::record::{Record, HowItWasStopped, WhatWasHappening};
use crate::app_data::{AppData, LoadError, SaveError};

use super::utils::time_utils;
use super::style::ButtonStyle;
use super::builder::{UiBuilder, ColumnAlignment};


#[derive(Default, Clone)]
struct WindowUiEditState {
    record: Record,
    // widgets
    description_state: text_input::State,
    crash_state: button::State,
    killed_state: button::State,
    clear_state: button::State,
}


#[derive(Default)]
struct WindowUiState {
    layout_debug: bool,
    last_tick: Option<DateTime<Utc>>,
    edit: WindowUiEditState,
    // layout
    records_scroll_state: scrollable::State,
}


#[derive(Default)]
pub struct MainWindow {
    data: Option<AppData>,
    ui: WindowUiState,
}


#[derive(Debug, Clone)]
pub enum Message {
    DataLoaded(Result<AppData, LoadError>),
    Saved(Result<(), SaveError>),
    Tick(DateTime<Utc>),
    EventOccurred(Event),
    WhatChanged(WhatWasHappening),
    DescriptionEdited(String),
    FrozenToggled(bool),
    BusyToggled(bool),
    CrashClicked,
    KilledClicked,
    ClearClicked,
}


impl Application for MainWindow {
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
            Message::WhatChanged(what) => {
                self.ui.edit.record.what = Some(what);
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
            Message::CrashClicked => {
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
        let builder = UiBuilder::new();
        let now = self.ui.last_tick.unwrap_or_else(Utc::now);
        let duration_format = |d: Duration| format!(
            "{:02}:{:02}:{:02}",
            d.num_hours(),
            d.num_minutes(),
            d.num_seconds());

        let records_len = self.data.as_ref().map_or(0, |data| data.records.len());

        let frozen_spent = match self.ui.edit.record.frozen {
            Some(when) => format!(" {} ago", duration_format(now - when)),
            None => String::new()
        };

        let busy_spent = match self.ui.edit.record.busy {
            Some(when) => format!(" {} ago", duration_format(now - when)),
            None => String::new()
        };

        //
        let crash_button = builder.button(
            &mut self.ui.edit.crash_state,
            "Crashed",
            ButtonStyle::Secondary,
            Message::CrashClicked);

        let killed_button = builder.button(
            &mut self.ui.edit.killed_state,
            "Killed",
            ButtonStyle::Secondary,
            Message::KilledClicked);

        let history_right_part =
            if records_len > 0 {
               vec![builder.button(
                   &mut self.ui.edit.clear_state,
                   "Clear",
                   ButtonStyle::Danger,
                   Message::ClearClicked)]
            }
            else { vec![] };

        let rows = vec![
            builder.title("Record"),
            builder.item_vspacer(),
            builder.two_col_row(
                vec![builder.radio(WhatWasHappening::Typing,
                          "Typing",
                          self.ui.edit.record.what,
                          ButtonStyle::Secondary,
                          Message::WhatChanged)],
                vec![builder.radio(WhatWasHappening::Running,
                          "Running",
                          self.ui.edit.record.what,
                          ButtonStyle::Secondary,
                          Message::WhatChanged)],
                ColumnAlignment::Left
            ),
            builder.item_vspacer(),
            builder.two_col_row(
                vec![builder.radio(WhatWasHappening::Testing,
                          "Testing",
                          self.ui.edit.record.what,
                          ButtonStyle::Secondary,
                          Message::WhatChanged)],
                vec![builder.radio(WhatWasHappening::Debugging,
                          "Debugging",
                          self.ui.edit.record.what,
                          ButtonStyle::Secondary,
                          Message::WhatChanged)],
                ColumnAlignment::Left
            ),
            builder.item_vspacer(),
            builder.input(&mut self.ui.edit.description_state,
                          "Description...",
                          &self.ui.edit.record.description,
                          Message::DescriptionEdited),
            builder.item_vspacer(),
            builder.form_row(
                builder.checkbox(self.ui.edit.record.frozen.is_some(),
                                 "Frozen",
                                 ButtonStyle::Secondary,
                                 Message::FrozenToggled),
                builder.label(frozen_spent),
            ),
            builder.item_vspacer(),
            builder.form_row(
                builder.checkbox(self.ui.edit.record.busy.is_some(),
                                 "Busy",
                                 ButtonStyle::Secondary,
                                 Message::BusyToggled),
                builder.label(busy_spent),
            ),
            builder.item_vspacer(),
            builder.two_col_row(vec![], vec![crash_button, killed_button], ColumnAlignment::Right),
            builder.section_vspacer(),
            builder.two_col_row(
                vec![builder.title(format!("History ({})", records_len))],
                history_right_part,
                ColumnAlignment::Outward),
            builder.item_vspacer(),
            match &self.data {
                None => {
                    builder.placeholder("No records.")
                },
                Some(ref data) if data.records.is_empty() => {
                    builder.placeholder("No records.")
                },
                Some(ref data) => {
                    builder.list(&mut self.ui.records_scroll_state,
                                 data.records .iter()
                                     .map(|record| Self::make_entry(&builder, record))
                                     .collect())
                }
            }
        ];

        builder.root(self.ui.layout_debug, rows)
    }
}

impl MainWindow {

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

    fn make_entry<'a, 'b>(builder: &UiBuilder, entry: &'a Record) -> UiElement!(for<'b>) {
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

        builder.label(text)
    }

}
