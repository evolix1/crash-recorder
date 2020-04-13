use iced::{button, Background, Color, Vector};

pub const SECTION_GAP: u16 = 30;
pub const ITEM_GAP: u16 = 18;
pub const BUTTON_GAP: u16 = 12;
pub const LIST_GAP: u16 = 5;

pub const FORM_LAYOUT_LEFT_WIDTH: u16 = 120;


pub enum ButtonStyle {
    Primary,
    Secondary,
    Danger,
}

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        let background = Some(Background::Color(match self {
            Self::Primary => Color::from_rgb(0.11, 0.42, 0.87),
            Self::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
            Self::Danger => Color::from_rgb8(157, 12, 12),
        }));

        button::Style {
            background,
            border_radius: 4,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}
