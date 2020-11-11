use iced_native::{Background, Color, Vector};

pub struct Style {
   pub shadow_offset: Vector,
   pub background: Option<Background>,
   pub border_radius: u16,
   pub border_width: u16,
   pub border_color: Color,
   pub text_color: Color,
}

impl std::default::Default for Style {
   fn default() -> Self {
      Self {
         shadow_offset: Vector::default(),
         background: None,
         border_radius: 10,
         border_width: 0,
         border_color: Color::TRANSPARENT,
         text_color: Color::BLACK,
      }
   }
}

pub trait StyleSheet {
   fn active(&self) -> Style;

   fn hovered(&self) -> Style {
      Style {
         shadow_offset: self.active().shadow_offset + Vector::new(0., 1.),
         ..self.active()
      }
   }

   fn pressed(&self) -> Style {
      self.active()
   }

   fn disabled(&self) -> Style {
      let active = self.active();
      Style {
         shadow_offset: Vector::default(),
         background: active.background.map(|bg| match bg {
            Background::Color(color) => Background::Color(Color {
               a: color.a * 0.5,
               ..color
            })
         }),
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
         background: Some(Background::Color([0.87, 0.87, 0.87].into())),
         border_color: [0.7, 0.7, 0.7].into(),
         ..Style::default()
      }
   }
}

impl std::default::Default for Box<dyn StyleSheet> {
   fn default() -> Self {
      Box::new(Default)
   }
}

impl<T> From<T> for Box<dyn StyleSheet> 
where T: 'static + StyleSheet {
   fn from(style: T) -> Self {
      Box::new(style)
   }
}