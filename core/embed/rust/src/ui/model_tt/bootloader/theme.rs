use crate::{
    alpha,
    ui::{
        component::text::TextStyle,
        display::{Color, Font},
        model_tt::{
            component::{ButtonStyle, ButtonStyleSheet},
            theme::{FG, GREEN, GREEN_DARK, GREY_LIGHT, RED, RED_DARK, WHITE},
        },
    },
};

pub const BLD_BG: Color = Color::rgb(0x00, 0x17, 0xA3);
pub const BLD_FG: Color = WHITE;
pub const BLD_BTN_MENU_COLOR: Color = Color::rgba(BLD_BG, 0xFF, 0xFF, 0xFF, alpha!(0.22));
pub const BLD_BTN_MENU_COLOR_ACTIVE: Color = Color::rgba(BLD_BG, 0xFF, 0xFF, 0xFF, alpha!(0.11));
pub const BLD_BTN_MENUITEM_COLOR: Color = Color::rgba(BLD_BG, 0xFF, 0xFF, 0xFF, alpha!(0.33));
pub const BLD_BTN_MENUITEM_COLOR_ACTIVE: Color =
    Color::rgba(BLD_BG, 0xFF, 0xFF, 0xFF, alpha!(0.11));
pub const BLD_TITLE_COLOR: Color = Color::rgba(BLD_BG, 0xFF, 0xFF, 0xFF, alpha!(0.75));

// Commonly used corner radius (i.e. for buttons).
pub const RADIUS: u8 = 2;

// Size of icons in the UI (i.e. inside buttons).
pub const ICON_SIZE: i32 = 16;

// UI icons.
pub const ICON_CANCEL: &[u8] = include_res!("model_tt/res/cancel.toif");
pub const ICON_CONFIRM: &[u8] = include_res!("model_tt/res/confirm.toif");

// BLD icons
pub const CLOSE: &[u8] = include_res!("model_tt/res/close.toif");
pub const RESET: &[u8] = include_res!("model_tt/res/reset.toif");
pub const FWINFO: &[u8] = include_res!("model_tt/res/fwinfo.toif");
pub const REBOOT: &[u8] = include_res!("model_tt/res/reboot.toif");
pub const MENU: &[u8] = include_res!("model_tt/res/menu.toif");
pub const RECEIVE: &[u8] = include_res!("model_tt/res/receive.toif");

pub fn button_cancel() -> ButtonStyleSheet {
    ButtonStyleSheet {
        normal: &ButtonStyle {
            font: Font::BOLD,
            text_color: WHITE,
            button_color: RED,
            background_color: WHITE,
            border_color: WHITE,
            border_radius: RADIUS,
            border_width: 0,
        },
        active: &ButtonStyle {
            font: Font::BOLD,
            text_color: WHITE,
            button_color: RED_DARK,
            background_color: WHITE,
            border_color: WHITE,
            border_radius: RADIUS,
            border_width: 0,
        },
        disabled: &ButtonStyle {
            font: Font::BOLD,
            text_color: GREY_LIGHT,
            button_color: RED,
            background_color: WHITE,
            border_color: WHITE,
            border_radius: RADIUS,
            border_width: 0,
        },
    }
}

pub fn button_confirm() -> ButtonStyleSheet {
    ButtonStyleSheet {
        normal: &ButtonStyle {
            font: Font::BOLD,
            text_color: WHITE,
            button_color: GREEN,
            background_color: WHITE,
            border_color: WHITE,
            border_radius: RADIUS,
            border_width: 0,
        },
        active: &ButtonStyle {
            font: Font::BOLD,
            text_color: WHITE,
            button_color: GREEN_DARK,
            background_color: WHITE,
            border_color: WHITE,
            border_radius: RADIUS,
            border_width: 0,
        },
        disabled: &ButtonStyle {
            font: Font::BOLD,
            text_color: FG,
            button_color: GREEN,
            background_color: FG,
            border_color: FG,
            border_radius: RADIUS,
            border_width: 0,
        },
    }
}

pub fn button_bld_menu() -> ButtonStyleSheet {
    ButtonStyleSheet {
        normal: &ButtonStyle {
            font: Font::BOLD,
            text_color: BLD_FG,
            button_color: BLD_BTN_MENU_COLOR,
            background_color: BLD_BG,
            border_color: BLD_BG,
            border_radius: 4,
            border_width: 0,
        },
        active: &ButtonStyle {
            font: Font::BOLD,
            text_color: BLD_FG,
            button_color: BLD_BTN_MENU_COLOR_ACTIVE,
            background_color: BLD_BG,
            border_color: BLD_BG,
            border_radius: 4,
            border_width: 0,
        },
        disabled: &ButtonStyle {
            font: Font::BOLD,
            text_color: GREY_LIGHT,
            button_color: BLD_BTN_MENU_COLOR,
            background_color: BLD_BG,
            border_color: BLD_BG,
            border_radius: 4,
            border_width: 0,
        },
    }
}

pub fn button_bld_menu_item() -> ButtonStyleSheet {
    ButtonStyleSheet {
        normal: &ButtonStyle {
            font: Font::BOLD,
            text_color: BLD_FG,
            button_color: BLD_BTN_MENUITEM_COLOR,
            background_color: BLD_BG,
            border_color: BLD_BG,
            border_radius: 4,
            border_width: 0,
        },
        active: &ButtonStyle {
            font: Font::BOLD,
            text_color: BLD_FG,
            button_color: BLD_BTN_MENUITEM_COLOR_ACTIVE,
            background_color: BLD_BG,
            border_color: BLD_BG,
            border_radius: 4,
            border_width: 0,
        },
        disabled: &ButtonStyle {
            font: Font::BOLD,
            text_color: GREY_LIGHT,
            button_color: BLD_BTN_MENUITEM_COLOR,
            background_color: BLD_BG,
            border_color: BLD_BG,
            border_radius: 4,
            border_width: 0,
        },
    }
}

pub const TEXT_NORMAL: TextStyle =
    TextStyle::new(Font::NORMAL, BLD_FG, BLD_BG, GREY_LIGHT, GREY_LIGHT);
pub const TEXT_BOLD: TextStyle = TextStyle::new(Font::BOLD, BLD_FG, BLD_BG, GREY_LIGHT, GREY_LIGHT);
pub const TEXT_MEDIUM: TextStyle =
    TextStyle::new(Font::MEDIUM, BLD_FG, BLD_BG, GREY_LIGHT, GREY_LIGHT);
