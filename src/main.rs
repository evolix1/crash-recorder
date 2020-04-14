mod record;
mod app_data;
mod ui;

use ui::app::AppUi;

fn main() {
    <AppUi as iced::Application>::run(
        iced::settings::Settings {
            window: iced::window::Settings {
                size: (400, 600),
                resizable: true,
                decorations: true,
            },
            ..iced::settings::Settings::default()
        })
}
