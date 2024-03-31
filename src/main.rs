mod cell;
use cell::cell;

use iced::widget::{column, container};
use iced::{Alignment, Element, Length};

pub fn main() -> iced::Result {
    iced::run("Custom Widget - Iced", Example::update, Example::view)
}

struct Example<'a> {
    content: &'a str,
    side: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CellPressed
}

impl<'a> Example<'a> {
    fn new() -> Self {
        Example { 
            content: "X",
            side: 50.0
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CellPressed => {
                self.side *= 1.1;
                if self.content == "X" {
                    self.content = "O"
                } else {
                    self.content = "X"
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            cell(self.content, self.side, Message::CellPressed),
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

impl<'a> Default for Example<'a> {
    fn default() -> Self {
        Self::new()
    }
}
