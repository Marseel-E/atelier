use crate::ui::layout::master_layout;
use iced::Element;
use muda::Menu;

pub struct Atelier {
    _system_menu: Option<Menu>,
}

impl Default for Atelier {
    fn default() -> Self {
        Self { _system_menu: None }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InitMenu,
}

impl Atelier {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::InitMenu => {
                let menu = crate::system_menu::init_menu();

                self._system_menu = Some(menu);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        master_layout(self)
    }
}
