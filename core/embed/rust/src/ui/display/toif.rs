use crate::{
    trezorhal,
    trezorhal::{
        display::{icon, image, ToifFormat},
        uzlib::{UzlibContext, UZLIB_WINDOW_SIZE},
    },
    ui::{
        constant,
        display::{get_color_table, get_offset, pixeldata, pixeldata_dirty, set_window},
        geometry::{Alignment2D, Offset, Point, Rect},
    },
};
use heapless::String;

use super::Color;

/// Storing the icon together with its name
/// Needs to be a tuple-struct, so it can be made `const`
#[derive(Debug, Clone, Copy)]
pub struct NamedToif(pub &'static [u8], pub &'static str);

impl NamedToif {
    pub const fn new(icon: &'static [u8], name: &'static str) -> Self {
        Self(icon, name)
    }
}

pub fn toif_info(data: &[u8]) -> Option<(Offset, ToifFormat)> {
    if let Ok(info) = trezorhal::display::toif_info(data) {
        Some((
            Offset::new(
                unwrap!(info.width.try_into()),
                unwrap!(info.height.try_into()),
            ),
            info.format,
        ))
    } else {
        None
    }
}

/// Aborts if the TOIF file does not have the correct grayscale flag, do not use
/// with user-supplied inputs.
pub fn toif_info_ensure(data: &[u8], format: ToifFormat) -> (Offset, &[u8]) {
    let info = unwrap!(trezorhal::display::toif_info(data), "Invalid TOIF data");
    assert_eq!(info.format, format);
    let size = Offset::new(
        unwrap!(info.width.try_into()),
        unwrap!(info.height.try_into()),
    );
    let payload = &data[12..]; // Skip TOIF header.
    (size, payload)
}

pub fn icon_precise(icon: &Icon, center: Point, fg_color: Color, bg_color: Color) {
    let r = Rect::from_center_and_size(center, icon.size());

    let area = r.translate(get_offset());
    let clamped = area.clamp(constant::screen());
    let colortable = get_color_table(fg_color, bg_color);

    set_window(clamped);

    let mut dest = [0_u8; 1];

    let mut window = [0; UZLIB_WINDOW_SIZE];
    let mut ctx = UzlibContext::new(icon.data(), Some(&mut window));

    for py in area.y0..area.y1 {
        for px in area.x0..area.x1 {
            let p = Point::new(px, py);
            let x = p.x - area.x0;

            if clamped.contains(p) {
                if x % 2 == 0 {
                    unwrap!(ctx.uncompress(&mut dest), "Decompression failed");
                    pixeldata(colortable[(dest[0] & 0xF) as usize]);
                } else {
                    pixeldata(colortable[(dest[0] >> 4) as usize]);
                }
            } else if x % 2 == 0 {
                //continue unzipping but dont write to display
                unwrap!(ctx.uncompress(&mut dest), "Decompression failed");
            }
        }
    }

    pixeldata_dirty();
}

/// Holding toif data and allowing it to draw itself.
/// Lots of draw methods exist so that we can easily
/// "glue" the toif together with other elements
/// (text, display boundary, etc.) according to their position.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Toif {
    data: &'static [u8],
    // Text is useful for debugging purposes.
    text: &'static str,
    size: Offset,
    format: ToifFormat,
}

impl Toif {
    pub fn new(named_toif: NamedToif) -> Self {
        let info = toif_info(named_toif.0);
        let info = unwrap!(info);
        Self {
            data: named_toif.0[12..].as_ref(),
            text: named_toif.1,
            size: info.0,
            format: info.1,
        }
    }

    pub fn from_slice(data: &'static [u8]) -> Self {
        let info = toif_info(data);
        let info = unwrap!(info);
        Self {
            data: data[12..].as_ref(),
            text: "",
            size: info.0,
            format: info.1,
        }
    }

    pub fn ensure(&self, format: ToifFormat) {
        assert_eq!(self.format, format);
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Icon {
    toif: Toif,
    area: Rect,
    precise_x: bool,
}

impl Icon {
    pub fn new(named_toif: NamedToif) -> Self {
        let toif = Toif::new(named_toif);
        Self::check(&toif);
        Self {
            toif,
            area: Rect::zero(),
            precise_x: false,
        }
    }

    pub fn from_slice(data: &'static [u8]) -> Self {
        let toif = Toif::from_slice(data);
        Self::check(&toif);
        Self {
            toif,
            area: Rect::zero(),
            precise_x: false,
        }
    }

    pub fn with_precise_x(mut self) -> Self {
        self.precise_x = true;
        self
    }

    fn check(toif: &Toif) {
        if toif.format != ToifFormat::GrayScaleEH {
            let s = "Icon format is not GrayScaleEH";
            let e: Result<(), ()> = Err(());
            let mut msg: String<64> = String::new();
            unwrap!(msg.push_str("Icon '"), s);
            unwrap!(msg.push_str(toif.text), s);
            unwrap!(msg.push_str("' is not in GrayScaleEH format"), s);
            unwrap!(e, msg.as_ref());
        }
    }

    pub fn width(&self) -> i16 {
        self.toif.size.x
    }

    pub fn height(&self) -> i16 {
        self.toif.size.y
    }

    pub fn size(&self) -> Offset {
        self.toif.size
    }

    pub fn data(&self) -> &[u8] {
        self.toif.data
    }

    /// Display the icon with baseline Point, aligned according to the
    /// `alignment` argument.
    pub fn draw(&self, baseline: Point, alignment: Alignment2D, fg_color: Color, bg_color: Color) {
        let r = Rect::snap(baseline, self.size(), alignment);

        if self.precise_x {
            icon_precise(self, r.center(), fg_color, bg_color);
        } else {
            icon(
                r.x0,
                r.y0,
                r.width(),
                r.height(),
                self.toif.data,
                fg_color.into(),
                bg_color.into(),
            );
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Image {
    toif: Toif,
    pub area: Rect,
    precise_x: bool,
}

impl Image {
    pub fn new(named_toif: NamedToif) -> Self {
        let toif = Toif::new(named_toif);
        Self::check(&toif);
        Self {
            toif,
            area: Rect::zero(),
            precise_x: false,
        }
    }

    pub fn from_slice(data: &'static [u8]) -> Self {
        let toif = Toif::from_slice(data);
        Self::check(&toif);
        Self {
            toif,
            area: Rect::zero(),
            precise_x: false,
        }
    }

    fn check(toif: &Toif) {
        if toif.format != ToifFormat::FullColorLE {
            let s = "Image format is not FullColorLE";
            let e: Result<(), ()> = Err(());
            let mut msg: String<64> = String::new();
            unwrap!(msg.push_str("Image '"), s);
            unwrap!(msg.push_str(toif.text), s);
            unwrap!(msg.push_str("' is not in FullColorLE format"), s);
            unwrap!(e, msg.as_ref());
        }
    }

    pub fn width(&self) -> i16 {
        self.toif.size.x
    }

    pub fn height(&self) -> i16 {
        self.toif.size.y
    }

    pub fn size(&self) -> Offset {
        self.toif.size
    }

    pub fn data(&self) -> &[u8] {
        self.toif.data
    }

    /// Display the icon with baseline Point, aligned according to the
    /// `alignment` argument.
    pub fn draw(&self, baseline: Point, alignment: Alignment2D) {
        let r = Rect::snap(baseline, self.size(), alignment);
        image(r.x0, r.y0, r.width(), r.height(), self.data());
    }
}
