use crate::app::{Atelier, Message};
use iced::widget::{Container, column, container, row, text};
use iced::{Element, Length};

pub fn master_layout(_app: &Atelier) -> Element<'_, Message> {
    // Header
    let quick_access = placeholder_panel("Quick Access Panel");
    let workspace_settings = placeholder_panel("Workspace Settings").width(Length::Fixed(150.0));

    let header = row![quick_access, workspace_settings].height(Length::Fixed(30.0));

    // Body
    let tools_panel = placeholder_panel("Tools").width(Length::Fixed(75.0));
    let object_settings = placeholder_panel("Object Settings").width(Length::Fixed(250.0));

    // Main
    let viewport = placeholder_panel("Viewport")
        .width(Length::Fill)
        .height(Length::Fill);

    let timeline_panel = placeholder_panel("Layers & Animation Timeline")
        .height(Length::Fixed(150.0))
        .width(Length::Fill);

    let main = column![viewport, timeline_panel].width(Length::Fill);

    let body = row![tools_panel, main, object_settings].height(Length::Fill);

    column![header, body].into()
}

fn placeholder_panel<'a>(label: &'a str) -> Container<'a, Message> {
    container(text(label).size(16))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| {
            container::Style::default().border(iced::Border {
                color: iced::Color::from_rgb(0.5, 0.5, 0.5),
                width: 1.0,
                radius: 0.0.into(),
            })
        })
}
