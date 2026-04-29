mod app;
mod system_menu;
mod ui;

use app::{Atelier, Message};
use iced::Task;

pub fn main() -> iced::Result {
    iced::application(
        || {
            (
                Atelier::default(),
                Task::perform(async {}, |()| Message::InitMenu),
            )
        },
        Atelier::update,
        Atelier::view,
    )
    .title("Atelier™")
    .window(iced::window::Settings {
        size: iced::Size::new(1200.0, 800.0),
        min_size: Some(iced::Size::new(900.0, 600.0)),
        position: iced::window::Position::Centered,
        ..Default::default()
    })
    .antialiasing(true)
    .run()
}
