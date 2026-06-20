use iced::border::Radius;
use iced::font::{Family, Stretch, Style as FontStyle, Weight};
use iced::theme::Palette;
use iced::widget::button::{Status as ButtonStatus, Style as ButtonStyle};
use iced::widget::container::Style as ContainerStyle;
use iced::widget::scrollable::{
    default as scrollable_default, Rail, Scroller, Status as ScrollableStatus,
    Style as ScrollableStyle,
};
use iced::widget::text_input::{Status as TextInputStatus, Style as TextInputStyle};
use iced::{border, Background, Border, Color, Font, Shadow, Theme};
use once_cell::sync::Lazy;

pub fn c(hex: &str) -> Color {
    hex.parse().unwrap()
}

pub fn is_dark(t: &Theme) -> bool {
    t.palette().background.r < 0.5
}

// ── Fonts ────────────────────────────────────────────────────────────────────

pub const SANS: Font = Font {
    family: Family::Name("IBM Plex Sans"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: FontStyle::Normal,
};
pub const SANS_MEDIUM: Font = Font {
    family: Family::Name("IBM Plex Sans"),
    weight: Weight::Medium,
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

// ── Themes ───────────────────────────────────────────────────────────────────

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

// ── Ping colour ───────────────────────────────────────────────────────────────

pub fn ping_color(ms: u64) -> Color {
    if ms < 80 {
        c("#3DD68C")
    } else if ms <= 200 {
        c("#E0B23C")
    } else {
        c("#E5604D")
    }
}

// ── Buttons ──────────────────────────────────────────────────────────────────

pub fn button_primary(t: &Theme, status: ButtonStatus) -> ButtonStyle {
    let dark = is_dark(t);
    let accent      = if dark { c("#3DD68C") } else { c("#18A862") };
    let accent_hov  = if dark { c("#34C27E") } else { c("#138A52") };
    let accent_prs  = if dark { c("#2BAD6F") } else { c("#0F7040") };
    let text_en     = if dark { c("#08110B") } else { c("#FFFFFF") };
    let dis_bg      = if dark { c("#1A1F27") } else { c("#DDE2E8") };
    let dis_text    = if dark { c("#5C636F") } else { c("#A0A7B1") };

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
    let bg      = if dark { c("#E5604D") } else { c("#CC3A28") };
    let bg_hov  = if dark { c("#CC4A38") } else { c("#B33525") };
    let bg_prs  = if dark { c("#B33525") } else { c("#982A1E") };

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
    let bg        = if dark { c("#181D25") } else { c("#FFFFFF") };
    let bg_hov    = if dark { c("#1F2630") } else { c("#F2F4F7") };
    let bg_prs    = if dark { c("#232A34") } else { c("#E8ECF0") };
    let text      = if dark { c("#A2ABBA") } else { c("#3A4049") };
    let text_hov  = if dark { c("#E8EBF0") } else { c("#161A20") };
    let border_c  = if dark { c("#232A34") } else { c("#DDE2E8") };

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

pub fn add_button_style(t: &Theme, status: ButtonStatus) -> ButtonStyle {
    let dark = is_dark(t);
    let bg       = if dark { c("#181D25") } else { c("#F6F7F9") };
    let bg_hov   = if dark { c("#1F2630") } else { c("#EEF0F3") };
    let bg_prs   = if dark { c("#232A34") } else { c("#E5E8EC") };
    let text     = if dark { c("#6B7480") } else { c("#5B6470") };
    let text_hov = if dark { c("#E8EBF0") } else { c("#18A862") };
    let border_c = if dark { c("#232A34") } else { c("#E1E5EA") };

    let base = ButtonStyle {
        background: Some(Background::Color(bg)),
        text_color: text,
        border: Border { color: border_c, width: 1.0, radius: Radius::from(7) },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        ButtonStatus::Hovered => ButtonStyle { background: Some(Background::Color(bg_hov)), text_color: text_hov, ..base },
        ButtonStatus::Pressed => ButtonStyle { background: Some(Background::Color(bg_prs)), ..base },
        _ => base,
    }
}

pub fn trash_button_style(t: &Theme, status: ButtonStatus) -> ButtonStyle {
    let dark = is_dark(t);
    let red_text = if dark { c("#E5604D") } else { c("#CC3A28") };
    let idle_text = if dark { c("#6B7480") } else { c("#8A929E") };
    match status {
        ButtonStatus::Hovered => ButtonStyle {
            background: Some(Background::Color(Color { r: 0.898, g: 0.376, b: 0.302, a: 0.15 })),
            text_color: red_text,
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radius::from(6) },
            shadow: Shadow::default(),
            snap: false,
        },
        ButtonStatus::Pressed => ButtonStyle {
            background: Some(Background::Color(Color { r: 0.898, g: 0.376, b: 0.302, a: 0.25 })),
            text_color: red_text,
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radius::from(6) },
            shadow: Shadow::default(),
            snap: false,
        },
        _ => ButtonStyle {
            background: None,
            text_color: idle_text,
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: Radius::from(6) },
            shadow: Shadow::default(),
            snap: false,
        },
    }
}

// ── Scrollable ───────────────────────────────────────────────────────────────

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

// ── Containers ───────────────────────────────────────────────────────────────

pub fn app_bg_style(t: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Some(Background::Color(if is_dark(t) { c("#0E1116") } else { c("#F0F1F3") })),
        ..Default::default()
    }
}

pub fn sidebar_style(t: &Theme) -> ContainerStyle {
    let dark = is_dark(t);
    ContainerStyle {
        background: Some(Background::Color(if dark { c("#0B0E13") } else { c("#FFFFFF") })),
        border: Border { color: if dark { c("#1A1F27") } else { c("#E1E5EA") }, width: 1.0, radius: Radius::from(0) },
        ..Default::default()
    }
}

pub fn header_style(t: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Some(Background::Color(if is_dark(t) { c("#0E1116") } else { c("#FFFFFF") })),
        ..Default::default()
    }
}

pub fn status_bar_style(t: &Theme) -> ContainerStyle {
    header_style(t)
}

pub fn surface_card_style(t: &Theme) -> ContainerStyle {
    let dark = is_dark(t);
    ContainerStyle {
        background: Some(Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") })),
        border: Border { color: if dark { c("#232A34") } else { c("#E5E9EF") }, width: 1.0, radius: Radius::from(10) },
        ..Default::default()
    }
}

pub fn right_panel_style(t: &Theme) -> ContainerStyle {
    let dark = is_dark(t);
    ContainerStyle {
        background: Some(Background::Color(if dark { c("#0B0E13") } else { c("#FFFFFF") })),
        border: Border { color: if dark { c("#1A1F27") } else { c("#E1E5EA") }, width: 1.0, radius: Radius::from(0) },
        ..Default::default()
    }
}

pub fn section_box_style(_: &Theme) -> ContainerStyle {
    ContainerStyle { background: None, ..Default::default() }
}

// ── Text inputs ──────────────────────────────────────────────────────────────

pub fn text_input_style(t: &Theme, status: TextInputStatus) -> TextInputStyle {
    let dark = is_dark(t);
    let focused = matches!(status, TextInputStatus::Focused { .. });
    let accent = if dark { c("#3DD68C") } else { c("#18A862") };
    let border_def = if dark { c("#232A34") } else { c("#DDE2E8") };
    TextInputStyle {
        background: Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") }),
        border: Border { radius: 7.0.into(), width: 1.0, color: if focused { accent } else { border_def } },
        icon: Color::TRANSPARENT,
        placeholder: if dark { c("#5C636F") } else { c("#A0A7B1") },
        value: if dark { c("#E8EBF0") } else { c("#161A20") },
        selection: Color { r: 0.239, g: 0.839, b: 0.549, a: 0.25 },
    }
}

pub fn text_input_error_style(t: &Theme, status: TextInputStatus) -> TextInputStyle {
    let dark = is_dark(t);
    let focused = matches!(status, TextInputStatus::Focused { .. });
    let danger = if dark { c("#E5604D") } else { c("#CC3A28") };
    let danger_dim = if dark { c("#6B2020") } else { c("#EDA8A0") };
    TextInputStyle {
        background: Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") }),
        border: Border { radius: 7.0.into(), width: 1.5, color: if focused { danger } else { danger_dim } },
        icon: Color::TRANSPARENT,
        placeholder: if dark { c("#5C636F") } else { c("#A0A7B1") },
        value: danger,
        selection: Color { r: 0.898, g: 0.376, b: 0.302, a: 0.25 },
    }
}

// ── Text editor ──────────────────────────────────────────────────────────────

pub fn text_editor_style(t: &Theme, _: iced::widget::text_editor::Status) -> iced::widget::text_editor::Style {
    let dark = is_dark(t);
    iced::widget::text_editor::Style {
        background: Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") }),
        border: Border { color: if dark { c("#232A34") } else { c("#DDE2E8") }, width: 1.0, radius: 7.0.into() },
        placeholder: if dark { c("#5C636F") } else { c("#A0A7B1") },
        value: if dark { c("#E8EBF0") } else { c("#161A20") },
        selection: Color { r: 0.239, g: 0.839, b: 0.549, a: 0.25 },
    }
}

// ── Progress bar ──────────────────────────────────────────────────────────────

pub fn progress_bar_style(t: &Theme) -> iced::widget::progress_bar::Style {
    let dark = is_dark(t);
    iced::widget::progress_bar::Style {
        background: Background::Color(if dark { c("#1A1F27") } else { c("#E1E5EA") }),
        bar: Background::Color(if dark { c("#3DD68C") } else { c("#18A862") }),
        border: Border { radius: 2.0.into(), width: 0.0, color: Color::TRANSPARENT },
    }
}
