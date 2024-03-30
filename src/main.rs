mod cell;
use iced::widget::{column, container, slider, text};
use iced::{Alignment, Element, Length};

pub fn main() -> iced::Result {
    iced::run("Custom Widget - Iced", Example::update, Example::view)
}

struct Example {
    side: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
}

impl Example {
    fn new() -> Self {
        Example { side: 50.0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged(side) => {
                self.side = side;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            cell::cell(self.side),
            text(format!("Radius: {:.2}", self.side)),
            slider(1.0..=100.0, self.side, Message::RadiusChanged).step(0.01),
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
