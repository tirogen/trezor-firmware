use crate::ui::{
    component::{Component, Event, EventCtx, Never, Pad},
    display::{self, Font},
    geometry::{Point, Rect},
    model_tt::bootloader::theme::{BLD_BG, BLD_TITLE_COLOR},
};

use crate::ui::model_tt::constant::{HEIGHT, WIDTH};

pub struct Connect {
    bg: Pad,
    message: &'static str,
}

impl Connect {
    pub fn new(message: &'static str) -> Self {
        let mut instance = Self {
            bg: Pad::with_background(BLD_BG),
            message,
        };

        instance.bg.clear();
        instance
    }
}

impl Component for Connect {
    type Msg = Never;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.bg
            .place(Rect::new(Point::new(0, 0), Point::new(WIDTH, HEIGHT)));
        bounds
    }

    fn event(&mut self, _ctx: &mut EventCtx, _event: Event) -> Option<Self::Msg> {
        None
    }

    fn paint(&mut self) {
        self.bg.paint();
        display::text_top_left(
            Point::new(15, 24),
            self.message,
            Font::BOLD,
            BLD_TITLE_COLOR,
            BLD_BG,
        );
    }

    fn bounds(&self, _sink: &mut dyn FnMut(Rect)) {}
}
