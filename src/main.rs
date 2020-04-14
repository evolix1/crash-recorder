mod record;
mod app_data;
mod ui;

use ui::window::MainWindow;

fn main() {
    <MainWindow as iced::Application>::run(
        iced::settings::Settings {
            window: iced::window::Settings {
                size: (ui::style::WINDOW_WIDTH as u32, ui::style::WINDOW_HEIGHT as u32),
                resizable: true,
                decorations: true,
            },
            ..iced::settings::Settings::default()
        })
}
