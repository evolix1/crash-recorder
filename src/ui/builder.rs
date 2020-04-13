use iced::{
    button,
    Length, HorizontalAlignment, Align, Color,
    Text, Container, Column, Row, Button, Space, Checkbox,
};

use super::style::style;

pub struct UiBuilder;

impl UiBuilder {
    pub fn make_root<'a>(debug: bool,
                     items: Vec<UiElement!(for<'a>)>) -> UiElement!(for<'a>) {
        let mut central: UiElement!() =
            Column::with_children(items)
            .padding(20)
            .align_items(Align::Start) // horizontal align
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

    pub fn make_row(items: Vec<UiElement!()>) -> UiElement!() {
        Row::with_children(items)
            .width(Length::Fill)
            .align_items(Align::Center) // vertical align
            .into()
    }

    pub fn make_button_row<'a>(mut left: Vec<UiElement!(for<'a>)>,
                               mut right: Vec<UiElement!(for<'a>)>) -> UiElement!(for<'a>)
    {
        let mut row = Row::new()
            .width(Length::Fill)
            .align_items(Align::Center);

        for (i, element) in left.drain(..).enumerate() {
            if i > 0 {
                row = row.push(Self::make_hspace(style::BUTTON_GAP));
            }
            row = row.push(element);
        }

        row = row.push(Self::make_hfiller());

        for (i, element) in right.drain(..).enumerate() {
            if i > 0 {
                row = row.push(Self::make_hspace(style::BUTTON_GAP));
            }
            row = row.push(element);
        }

        row.into()
    }

    pub fn make_form_row<'a, L, R>(left: L, right: R) -> UiElement!(for<'a>)
        where
            L: Into<UiElement!(for<'a>)>,
            R: Into<UiElement!(for<'a>)>,
    {
        Row::new()
            .push(Container::new(left.into())
                  .width(Length::Units(style::FORM_LAYOUT_LEFT_WIDTH))
                  .max_width(style::FORM_LAYOUT_LEFT_WIDTH as u32))
            .push(Container::new(right.into())
                  .width(Length::Fill))
            .width(Length::Fill)
            .align_items(Align::Center) // vertical align
            .into()
    }

    pub fn make_title(title: &str) -> UiElement!() {
        Text::new(title)
            .horizontal_alignment(HorizontalAlignment::Center)
            .width(Length::Fill)
            .size(22)
            .color([0.1, 0.1, 0.1])
            .into()
    }

    pub fn make_placeholder(text: &str) -> UiElement!() {
        Text::new(text)
            .horizontal_alignment(HorizontalAlignment::Center)
            .width(Length::Fill)
            .size(18)
            .color([0.1, 0.1, 0.3])
            .into()
    }

    pub fn make_label<T>(label: T) -> UiElement!(for<'static>)
        where T: Into<String>
    {
        Text::new(label)
            .size(18) // font size
            .color([0.2, 0.2, 0.2]) // font color
            .into()
    }

    pub fn make_button<'a>(state: &'a mut button::State, text: &str,
                   msg: UiMessage!()) -> UiElement!(for<'a>) {
        Button::new(state, Text::new(text))
            .min_width(50)
            .min_height(20)
            .padding(10)
            .on_press(msg)
            .into()
    }

    pub fn make_checkbox(state: bool, text: &str, msg: fn(bool) -> UiMessage!()) -> UiElement!() {
        Checkbox::new(state, text, msg)
            .spacing(8)
            .text_size(18)
            .into()
    }

    pub fn make_vspace(space: u16) -> UiElement!(for<'static>) {
        Space::new(Length::Shrink, Length::Units(space)).into()
    }

    pub fn make_hspace(space: u16) -> UiElement!(for<'static>) {
        Space::new(Length::Units(space), Length::Shrink).into()
    }

    pub fn make_hfiller() -> UiElement!(for<'static>) {
        Space::new(Length::Fill, Length::Shrink).into()
    }

}
