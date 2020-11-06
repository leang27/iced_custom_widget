use iced_native::{Background, Color, Vector};

pub struct Style {
   pub border_radius: u16,
   pub border_width: u16,
   pub border_color: Color,
   pub text_color: Color,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            border_radius: 10,
            border_width: 1,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
        }
    }
}

pub trait StyleSheet {
    fn active(&self) -> Style;

    fn hovered(&self) -> Style {
        self.active()
    }

    fn pressed(&self) -> Style {
        Style {
            text_color: Color::WHITE,
            ..self.active()
        }
    }

    fn disabled(&self) -> Style {
        let active = self.active();

        Style {
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}

struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style {
            border_radius: 10,
            border_width: 1,
            border_color: [0.8, 0.8, 0.8].into(),
            text_color: Color::BLACK,
        }
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}