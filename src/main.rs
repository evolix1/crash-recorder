mod record;
mod app_data;
mod ui;

use ui::app::AppUi;

fn main() {
    <AppUi as iced::Application>::run(iced::Settings::default())
}
