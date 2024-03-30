//! This example showcases a simple native custom widget that draws a circle.
mod circle {
    // For now, to implement a custom native widget you will need to add
    // `iced_native` and `iced_wgpu` to your dependencies.
    //
    // Then, you simply need to define your widget type and implement the
    // `iced_native::Widget` trait with the `iced_wgpu::Renderer`.
    //
    // Of course, you can choose to make the implementation renderer-agnostic,
    // if you wish to, by creating your own `Renderer` trait, which could be
    // implemented by `iced_wgpu` and other renderers.
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::text;
    use iced::advanced::widget::{self, Widget};
    use iced::mouse;
    use iced::{Border, Color, Element, Length, Rectangle, Size};

    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        const CONTENT: &'static str = "X";
        pub fn new(radius: f32) -> Self {
            Self { radius}
        }
    }

    pub fn circle(radius: f32) -> Circle {
        Circle::new(radius)
    }

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Circle
    where
        Renderer: renderer::Renderer + text::Renderer<Font = iced::Font>,
    {
        fn size(&self) -> Size<Length> {
            Size {
                width: Length::Shrink,
                height: Length::Shrink,
            }
        }

        fn layout(
            &self,
            _tree: &mut widget::Tree,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
        }

        fn draw(
            &self,
            _state: &widget::Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor: mouse::Cursor,
            _viewport: &Rectangle,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: Border{
                        width: self.radius,
                        color: Color::from_rgb(0.5, 0.2, 0.3),
                        radius: 0.0.into()
                    },
                    ..renderer::Quad::default()
                },
                Color::BLACK,
            );


            let content = iced::advanced::Text{
                content: Self::CONTENT,
                bounds: iced::Size{
                    width: layout.bounds().width,
                    height: layout.bounds().height
                },
                size: 30.0.into(),
                line_height: iced::widget::text::LineHeight::default(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                shaping: iced::widget::text::Shaping::default(),
                font: iced::Font::default(),
            };
            renderer.fill_text(content,
                layout.bounds().center(),
                Color::BLACK,
                *_viewport);
            
        }
    }


    impl<'a, Message, Theme, Renderer> From<Circle>
        for Element<'a, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer + text::Renderer<Font = iced::Font>,
    {
        fn from(circle: Circle) -> Self {
            Self::new(circle)
        }
    }
}

use circle::circle;
use iced::widget::{column, container, slider, text};
use iced::{Alignment, Element, Length};

pub fn main() -> iced::Result {
    iced::run("Custom Widget - Iced", Example::update, Example::view)
}

struct Example {
    radius: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
}

impl Example {
    fn new() -> Self {
        Example { radius: 50.0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged(radius) => {
                self.radius = radius;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            circle(self.radius),
            text(format!("Radius: {:.2}", self.radius)),
            slider(1.0..=100.0, self.radius, Message::RadiusChanged).step(0.01),
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
