use iced::{
    button, text_input, scrollable,
    Length, HorizontalAlignment, Align, Color,
    Container, Column, Row, Scrollable, 
    Text, TextInput, Button, Space, Checkbox, Radio,
};

use super::style;


#[derive(PartialEq)]
pub enum ColumnAlignment {
    Left,       //  [X..X..]
    Inward,     //  [..XX..]
    Outward,    //  [X....X]
    Right       //  [..X..X]
}


pub struct UiBuilder;

impl UiBuilder {
    pub fn new() -> UiBuilder {
        UiBuilder {}
    }

    pub fn root<'a>(&self, debug: bool,
                    items: Vec<UiElement!(for<'a>)>) -> UiElement!(for<'a>) {
        let mut central: UiElement!() =
            Column::with_children(items)
            .padding(20)
            .align_items(Align::Start) // horizontal align
            .width(Length::Units(style::WINDOW_WIDTH))
            .into();

        if debug {
            central = central.explain(Color::BLACK)
        }

        Container::new(central)
            .width(Length::Fill) // fill the window width
            .center_x()
            .into()
    }

    pub fn two_col_row<'a>(&self,
        mut left: Vec<UiElement!(for<'a>)>,
        mut right: Vec<UiElement!(for<'a>)>,
        align: ColumnAlignment) -> UiElement!(for<'a>)
    {
        let mut left_row = Row::new().align_items(Align::Center);
        let mut right_row = Row::new().align_items(Align::Center);
        let has_left = !left.is_empty();
        let has_right = !right.is_empty();

        if has_left && (align == ColumnAlignment::Inward || align == ColumnAlignment::Right) {
            left_row = left_row.push(Space::new(Length::Fill, Length::Shrink));
        }

        for (i, element) in left.drain(..).enumerate() {
            if i > 0 {
                left_row = left_row.push(self.action_hspacer());
            }
            left_row = left_row.push(element);
        }

        if has_right && (align == ColumnAlignment::Outward || align == ColumnAlignment::Right) {
            right_row = right_row.push(Space::new(Length::Fill, Length::Shrink));
        }

        for (i, element) in right.drain(..).enumerate() {
            if i > 0 {
                right_row = right_row.push(self.action_hspacer());
            }
            right_row = right_row.push(element);
        }

        let mut row = Row::new().width(Length::Fill).align_items(Align::Center);

        if has_left {
            row = row.push(left_row.width(Length::FillPortion(1)));
        }
            
        if has_left != has_right {
            row = row.push(Space::new(Length::Fill, Length::Shrink));
        }
            
        if has_right {
            row = row.push(right_row.width(Length::FillPortion(1)));
        }
        
        row.into()
    }

    pub fn form_row<'a, L, R>(&self, left: L, right: R) -> UiElement!(for<'a>)
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

    pub fn list<'a>(&self,
                    state: &'a mut scrollable::State,
                    items: Vec<UiElement!(for<'a>)>) -> UiElement!(for<'a>)
    {
        let central = Column::with_children(items)
            .width(Length::Fill)
            .spacing(style::LIST_GAP);

        Scrollable::new(state)
            .width(Length::Fill)
            .padding(5)
            .push(central)
            .into()
    }

    pub fn title<T>(&self, title: T) -> UiElement!(for<'static>) 
        where T: Into<String>
    {
        Text::new(title.into())
            .horizontal_alignment(HorizontalAlignment::Center)
            .width(Length::Fill)
            .font(style::FontStyle::Bold.into())
            .size(24)
            .color([0.1, 0.1, 0.1])
            .into()
    }

    pub fn placeholder<T>(&self, text: T) -> UiElement!(for<'static>) 
        where T: Into<String>
    {
        Text::new(text.into())
            .horizontal_alignment(HorizontalAlignment::Center)
            .width(Length::Fill)
            .font(style::FontStyle::Italic.into())
            .size(18)
            .color([0.1, 0.1, 0.3])
            .into()
    }

    pub fn label<T>(&self, label: T) -> UiElement!(for<'static>)
        where T: Into<String>
    {
        Text::new(label)
            .font(style::FontStyle::Regular.into())
            .size(18) // font size
            .color([0.2, 0.2, 0.2]) // font color
            .into()
    }

    pub fn input<'a>(&self,
                     state: &'a mut text_input::State,
                     placeholder: &'a str,
                     value: &'a str,
                     msg: fn(String) -> UiMessage!()) -> UiElement!(for<'a>) {
        TextInput::new(state, placeholder, value, msg)
            .size(16) // font size
            .padding(5)
            .width(Length::Fill)
            .into()
    }


    pub fn button<'a>(&self, state: &'a mut button::State,
                      text: &str,
                      btn_style: style::ButtonStyle,
                      msg: UiMessage!()) -> UiElement!(for<'a>) {
        Button::new(state, Text::new(text))
            .min_width(50)
            .min_height(20)
            .padding(10)
            .on_press(msg)
            .style(btn_style)
            .into()
    }

    pub fn checkbox<'a>(&self,
                        state: bool,
                        label: &'a str,
                        btn_style: style::ButtonStyle,
                        msg: fn(bool) -> UiMessage!()) -> UiElement!(for<'a>) {
        Checkbox::new(state, label, msg)
            .spacing(8)
            .text_size(18)
            .style(btn_style)
            .into()
    }

    pub fn radio<'a, T>(&self,
                        value: T,
                        label: &'a str,
                        current: Option<T>,
                        btn_style: style::ButtonStyle,
                        msg: fn(T) -> UiMessage!()) -> UiElement!(for<'a>) 
        where T: 'static + Eq + Copy 
    {
        Radio::new(value, label, current, msg)
            //.spacing(8)
            //.text_size(18)
            .style(btn_style)
            .into()
    }

    pub fn section_vspacer(&self) -> UiElement!(for<'static>) {
        Space::new(Length::Shrink, Length::Units(style::SECTION_GAP)).into()
    }

    pub fn item_vspacer(&self) -> UiElement!(for<'static>) {
        Space::new(Length::Shrink, Length::Units(style::ITEM_GAP)).into()
    }

    pub fn action_hspacer(&self) -> UiElement!(for<'static>) {
        Space::new(Length::Units(style::BUTTON_GAP), Length::Shrink).into()
    }

}
