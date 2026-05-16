#[allow(special_module_name)]
mod lib;
mod error;
mod types;
mod ui;
fn dark_theme(_: &ui::App) -> iced::Theme {
    iced::Theme::Dark
}
pub fn main() -> iced::Result {
    iced::application(
        || (ui::App::default(), iced::Task::none()),
        ui::App::update,
        ui::App::view,
    )
    .title("Slitigs-SP")
    .theme(dark_theme)               
    .run()
}