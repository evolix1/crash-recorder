use iced::{button, Background, Color, Vector, Font};

pub const SECTION_GAP: u16 = 30;
pub const ITEM_GAP: u16 = 12;
pub const BUTTON_GAP: u16 = 8;
pub const LIST_GAP: u16 = 5;

pub const FORM_LAYOUT_LEFT_WIDTH: u16 = 120;


#[cfg(not(target_os="windows"))]
macro_rules! font {
    ($modifier:expr) => {
        Font::External {
            name: concat!("LiberationSans", $modifier),
            bytes: include_bytes!(concat!("/usr/share/fonts/truetype/liberation/LiberationSans-", $modifier, ".ttf"))
        }
    }
}

#[cfg(target_os="windows")]
macro_rules! font {
    ($title:expr, $modifier:expr) => {
        Font::External {
            name: concat!("Arial", $title),
            bytes: include_bytes!(concat!("C:\\Windows\\Fonts\\Arial", $modifier, ".ttf"))
        }
    }
}


#[cfg(not(target_os="windows"))]
const FONTS: [Font; 4] = [
    font!("Regular"),
    font!("Italic"),
    font!("Bold"),
    font!("BoldItalic"),
];


#[cfg(target_os="windows")]
const FONTS: [Font; 4] = [
    font!("Regular", ""),
    font!("Italic", "i"),
    font!("Bold", "bd"),
    font!("BoldItalic", "bi"),
];


#[allow(unused)]
pub enum FontStyle {
    Regular,
    Italic,
    Bold,
    BoldItalic
}


impl Into<Font> for FontStyle {
    fn into(self) -> Font {
        match self {
            FontStyle::Regular => FONTS[0],
            FontStyle::Italic => FONTS[1],
            FontStyle::Bold => FONTS[2],
            FontStyle::BoldItalic => FONTS[3],
        }
    }
}


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
