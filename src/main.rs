mod cell;
use cell::cell;

use iced::widget::{column, container};
use iced::{Alignment, Element, Length};

pub fn main() -> iced::Result {
    iced::run("Custom Widget - Iced", Example::update, Example::view)
}

struct Example {
    side: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CellPressed
}

impl Example {
    fn new() -> Self {
        Example { side: 50.0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CellPressed => {
                self.side = self.side * 2.0;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            cell(self.side, Message::CellPressed),
        ]
        .padding(20)
        .spacing(20)
        .max_width(500)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::new()
    }
}
