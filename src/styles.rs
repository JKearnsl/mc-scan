use iced::font::{Family, Stretch, Style as FontStyle, Weight};
use iced::theme::Palette;
use iced::{Color, Font, Theme};
use once_cell::sync::Lazy;

pub fn c(hex: &str) -> Color {
    hex.parse().expect(&format!("parse hex color {}", hex))
}

pub fn is_dark(t: &Theme) -> bool {
    t.palette().background.r < 0.5
}

// Fonts

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

// Themes

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



