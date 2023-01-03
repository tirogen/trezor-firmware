use crate::ui::{
    component::{Component, Event, EventCtx, Never},
    display,
    display::{
        toif::{Icon, Image},
        Color,
    },
    geometry::{Offset, Point, Rect, CENTER},
};

impl Component for Image {
    type Msg = Never;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.area = bounds;
        self.area
    }

    fn event(&mut self, _ctx: &mut EventCtx, _event: Event) -> Option<Self::Msg> {
        None
    }

    fn paint(&mut self) {
        self.draw(self.area.center(), CENTER);
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        sink(Rect::from_center_and_size(self.area.center(), self.size()));
    }
}

#[cfg(feature = "ui_debug")]
impl crate::trace::Trace for Image {
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("Image");
        t.close();
    }
}

pub struct BlendedImage {
    bg: Icon,
    fg: Icon,
    bg_color: Color,
    fg_color: Color,
    area_color: Color,
    bg_top_left: Point,
    fg_offset: Offset,
}

impl BlendedImage {
    pub fn new(bg: Icon, fg: Icon, bg_color: Color, fg_color: Color, area_color: Color) -> Self {
        Self {
            bg,
            fg,
            bg_color,
            fg_color,
            area_color,
            bg_top_left: Point::zero(),
            fg_offset: Offset::zero(),
        }
    }

    fn paint_image(&self) {
        display::icon_over_icon(
            None,
            (&self.bg, self.bg_top_left.into(), self.bg_color),
            (&self.fg, self.fg_offset, self.fg_color),
            self.area_color,
        );
    }
}

impl Component for BlendedImage {
    type Msg = Never;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.bg_top_left = self.bg.size().snap(bounds.center(), CENTER);

        let ft_top_left = self.fg.size().snap(bounds.center(), CENTER);
        self.fg_offset = ft_top_left - self.bg_top_left;

        Rect::from_top_left_and_size(self.bg_top_left, self.bg.size())
    }

    fn event(&mut self, _ctx: &mut EventCtx, _event: Event) -> Option<Self::Msg> {
        None
    }

    fn paint(&mut self) {
        self.paint_image();
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        sink(Rect::from_top_left_and_size(
            self.bg_top_left,
            self.bg.size(),
        ));
    }
}

#[cfg(feature = "ui_debug")]
impl crate::trace::Trace for BlendedImage {
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("BlendedImage");
        t.close();
    }
}
