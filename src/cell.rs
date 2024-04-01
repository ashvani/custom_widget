use iced::advanced::graphics::core::event;
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::text;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{Border, Color, Element, Length, Rectangle, Size};

pub struct Cell<'a, Message> {
    content: &'a str,
    side: f32,
    on_click: Message,
}


impl<'a, Message> Cell<'a, Message> {
    pub fn new(
        content: &'a str,
        side: f32, 
        message: Message
        ) -> Self {
        Self {
            content,
            side,
            on_click: message,
        }
    }

    pub fn on_click(mut self, on_click: Message) -> Self {
        self.on_click = on_click;
        self 
    }
}

pub fn cell<Message>(content: &str, side: f32, message: Message, ) -> Cell<'_, Message> {
    Cell::new(content, side, message)
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Cell<'a, Message>
where
    Message: Clone,
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
        layout::Node::new(Size::new(self.side * 2.0, self.side * 2.0))
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
                    width: self.side,
                    color: Color::from_rgb(0.5, 0.2, 0.3),
                    radius: 0.0.into()
                },
                ..renderer::Quad::default()
            },
            Color::BLACK,
        );


        let size = self.side * 0.9;
        let content = iced::advanced::Text{
            content: self.content,
            bounds: iced::Size{
                width: layout.bounds().width,
                height: layout.bounds().height
            },
            size: size.into(),
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

    fn on_event(
            &mut self,
            _state: &mut widget::Tree,
            event: iced::Event,
            layout: Layout<'_>,
            cursor: iced::advanced::mouse::Cursor,
            _renderer: &Renderer,
            _clipboard: &mut dyn iced::advanced::Clipboard,
            shell: &mut iced::advanced::Shell<'_, Message>,
            _viewport: &Rectangle,
        ) -> iced::advanced::graphics::core::event::Status {
        if cursor.is_over(layout.bounds()) {
            match event {
                iced::Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                    shell.publish(self.on_click.clone());
                    event::Status::Captured
                }

                _ => event::Status::Ignored,
            }

        } else {
            event::Status::Ignored
        }
    }
}


impl<'a, Message, Theme, Renderer> From<Cell<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: renderer::Renderer + text::Renderer<Font = iced::Font>,
{
    fn from(cell: Cell<'a, Message>) -> Self {
        Self::new(cell)
    }
}
