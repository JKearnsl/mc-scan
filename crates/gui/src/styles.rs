use iced::font::{Family, Stretch, Style as FontStyle, Weight};
use iced::theme::Palette;
use iced::{Color, Font, Theme};
use once_cell::sync::Lazy;

pub const fn c(hex: &str) -> Color {
    let b = hex.as_bytes();
    Color {
        r: hex_pair(b[1], b[2]) as f32 / 255.0,
        g: hex_pair(b[3], b[4]) as f32 / 255.0,
        b: hex_pair(b[5], b[6]) as f32 / 255.0,
        a: 1.0,
    }
}

const fn hex_pair(hi: u8, lo: u8) -> u8 {
    hex_digit(hi) * 16 + hex_digit(lo)
}

const fn hex_digit(d: u8) -> u8 {
    match d {
        b'0'..=b'9' => d - b'0',
        b'a'..=b'f' => d - b'a' + 10,
        b'A'..=b'F' => d - b'A' + 10,
        _ => panic!("invalid hex digit in color literal"),
    }
}

pub fn is_dark(t: &Theme) -> bool {
    t.palette().background.r < 0.5
}

pub const SANS: Font = Font {
    family: Family::Name("IBM Plex Sans"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: FontStyle::Normal,
};
pub const SANS_SEMIBOLD: Font = Font {
    family: Family::Name("IBM Plex Sans"),
    weight: Weight::Semibold,
    stretch: Stretch::Normal,
    style: FontStyle::Normal,
};
pub const MONO: Font = Font {
    family: Family::Name("IBM Plex Mono"),
    weight: Weight::Medium,
    stretch: Stretch::Normal,
    style: FontStyle::Normal,
};
pub const MONO_SEMIBOLD: Font = Font {
    family: Family::Name("IBM Plex Mono"),
    weight: Weight::Semibold,
    stretch: Stretch::Normal,
    style: FontStyle::Normal,
};

pub static COLOR_THEME: Lazy<Theme> = Lazy::new(|| {
    Theme::custom(
        "mc-scan-dark".to_string(),
        Palette {
            background: c("#0E1116"),
            text: c("#E8EBF0"),
            primary: c("#3DD68C"),
            success: c("#3DD68C"),
            warning: c("#E0B23C"),
            danger: c("#E5604D"),
        },
    )
});

pub static COLOR_THEME_LIGHT: Lazy<Theme> = Lazy::new(|| {
    Theme::custom(
        "mc-scan-light".to_string(),
        Palette {
            background: c("#F0F1F3"),
            text: c("#161A20"),
            primary: c("#18A862"),
            success: c("#18A862"),
            warning: c("#D4900A"),
            danger: c("#CC3A28"),
        },
    )
});
