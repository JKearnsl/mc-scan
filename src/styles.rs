use iced::border::Radius;
use iced::font::{Family, Stretch, Style as FontStyle, Weight};
use iced::theme::Palette;
use iced::widget::button::{Status as ButtonStatus, Style as ButtonStyle};
use iced::widget::scrollable::{
    default as scrollable_default, Rail, Scroller, Status as ScrollableStatus,
    Style as ScrollableStyle,
};
use iced::{border, Background, Border, Color, Font, Shadow, Theme};
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

// Buttons

pub fn button_primary(t: &Theme, status: ButtonStatus) -> ButtonStyle {
    let dark = is_dark(t);
    let accent     = if dark { c("#3DD68C") } else { c("#18A862") };
    let accent_hov = if dark { c("#34C27E") } else { c("#138A52") };
    let accent_prs = if dark { c("#2BAD6F") } else { c("#0F7040") };
    let text_en    = if dark { c("#08110B") } else { c("#FFFFFF") };
    let dis_bg     = if dark { c("#1A1F27") } else { c("#DDE2E8") };
    let dis_text   = if dark { c("#5C636F") } else { c("#A0A7B1") };

    let base = ButtonStyle {
        background: Some(Background::Color(accent)),
        text_color: text_en,
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radius::from(10) },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        ButtonStatus::Hovered  => ButtonStyle { background: Some(Background::Color(accent_hov)), ..base },
        ButtonStatus::Pressed  => ButtonStyle { background: Some(Background::Color(accent_prs)), ..base },
        ButtonStatus::Disabled => ButtonStyle { background: Some(Background::Color(dis_bg)), text_color: dis_text, ..base },
        _ => base,
    }
}

pub fn button_danger(t: &Theme, status: ButtonStatus) -> ButtonStyle {
    let dark = is_dark(t);
    let bg     = if dark { c("#E5604D") } else { c("#CC3A28") };
    let bg_hov = if dark { c("#CC4A38") } else { c("#B33525") };
    let bg_prs = if dark { c("#B33525") } else { c("#982A1E") };

    let base = ButtonStyle {
        background: Some(Background::Color(bg)),
        text_color: c("#FFFFFF"),
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radius::from(10) },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        ButtonStatus::Hovered => ButtonStyle { background: Some(Background::Color(bg_hov)), ..base },
        ButtonStatus::Pressed => ButtonStyle { background: Some(Background::Color(bg_prs)), ..base },
        _ => base,
    }
}

pub fn button_secondary(t: &Theme, status: ButtonStatus) -> ButtonStyle {
    let dark = is_dark(t);
    let bg       = if dark { c("#181D25") } else { c("#FFFFFF") };
    let bg_hov   = if dark { c("#1F2630") } else { c("#F2F4F7") };
    let bg_prs   = if dark { c("#232A34") } else { c("#E8ECF0") };
    let text     = if dark { c("#A2ABBA") } else { c("#3A4049") };
    let text_hov = if dark { c("#E8EBF0") } else { c("#161A20") };
    let border_c = if dark { c("#232A34") } else { c("#DDE2E8") };

    let base = ButtonStyle {
        background: Some(Background::Color(bg)),
        text_color: text,
        border: Border { color: border_c, width: 1.0, radius: Radius::from(10) },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        ButtonStatus::Hovered => ButtonStyle { background: Some(Background::Color(bg_hov)), text_color: text_hov, ..base },
        ButtonStatus::Pressed => ButtonStyle { background: Some(Background::Color(bg_prs)), ..base },
        _ => base,
    }
}

pub fn icon_button_style(t: &Theme, status: ButtonStatus) -> ButtonStyle {
    let dark = is_dark(t);
    let bg       = if dark { c("#181D25") } else { c("#F6F7F9") };
    let bg_hov   = if dark { c("#1F2630") } else { c("#EEF0F3") };
    let bg_prs   = if dark { c("#232A34") } else { c("#E5E8EC") };
    let text     = if dark { c("#6B7480") } else { c("#5B6470") };
    let text_hov = if dark { c("#E8EBF0") } else { c("#161A20") };
    let border_c = if dark { c("#232A34") } else { c("#E1E5EA") };

    let base = ButtonStyle {
        background: Some(Background::Color(bg)),
        text_color: text,
        border: Border { color: border_c, width: 1.0, radius: Radius::from(10) },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        ButtonStatus::Hovered => ButtonStyle { background: Some(Background::Color(bg_hov)), text_color: text_hov, ..base },
        ButtonStatus::Pressed => ButtonStyle { background: Some(Background::Color(bg_prs)), ..base },
        _ => base,
    }
}

// Scrollable

pub fn scrollable_style(theme: &Theme, status: ScrollableStatus) -> ScrollableStyle {
    let dark = is_dark(theme);
    let scroller = Scroller {
        background: Background::Color(if dark { c("#232A34") } else { c("#C8CDD5") }),
        border: border::rounded(2),
    };
    let rail = Rail {
        background: Some(Background::Color(if dark { c("#1A1F27") } else { c("#E1E5EA") })),
        border: border::rounded(2),
        scroller,
    };
    ScrollableStyle {
        vertical_rail: rail,
        ..scrollable_default(theme, status)
    }
}

