mod cell;
use cell::cell;

use iced::widget::{row, column, container};
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
    CellPressed11,
    CellPressed12,
    CellPressed13,
    CellPressed21,
    CellPressed22,
    CellPressed23,
    CellPressed31,
    CellPressed32,
    CellPressed33
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
            Message::CellPressed11 => {
                if self.content == "X" {
                    self.content = "O"
                } else {
                    self.content = "X"
                }
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            row![
                cell(self.content, self.side, Message::CellPressed11),
                cell(self.content, self.side, Message::CellPressed12),
                cell(self.content, self.side, Message::CellPressed13)
            ].spacing(1),
            row![
                cell(self.content, self.side, Message::CellPressed21),
                cell(self.content, self.side, Message::CellPressed22),
                cell(self.content, self.side, Message::CellPressed23)
            ].spacing(1),
            row![
                cell(self.content, self.side, Message::CellPressed31),
                cell(self.content, self.side, Message::CellPressed32),
                cell(self.content, self.side, Message::CellPressed33)
            ].spacing(1)
            
        ]
        .padding(20)
        .spacing(1)
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
