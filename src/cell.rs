use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::text;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{Border, Color, Element, Length, Rectangle, Size};

pub struct Cell {
    side: f32,
}

impl Cell {
    const CONTENT: &'static str = "X";
    pub fn new(side: f32) -> Self {
        Self { side}
    }
}

pub fn cell(side: f32) -> Cell {
    Cell::new(side)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Cell
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
            content: Self::CONTENT,
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
}


impl<'a, Message, Theme, Renderer> From<Cell>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + text::Renderer<Font = iced::Font>,
{
    fn from(cell: Cell) -> Self {
        Self::new(cell)
    }
}
