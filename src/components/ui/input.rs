use iced::widget::{row, text, text_input};
use iced::Length::Fixed;
use iced::{Alignment, Background, Border, Color, Element, Fill, Padding, Theme};

use crate::styles::{c, is_dark, MONO, SANS};

pub fn labeled_input<'a, M: Clone + 'a>(
    label: &'a str,
    value: &'a str,
    placeholder: &'a str,
    on_change: impl Fn(String) -> M + 'a,
    error: bool,
) -> Element<'a, M> {
    let style: fn(&Theme, _) -> _ = if error { text_input_error_style } else { text_input_style };
    row![
        text(label).size(13).font(SANS).width(Fixed(90.0)).style(|t: &Theme| {
            iced::widget::text::Style {
                color: Some(if is_dark(t) { c("#A2ABBA") } else { c("#3A4049") }),
            }
        }),
        text_input(placeholder, value)
            .on_input(on_change)
            .padding(Padding::from([7, 10]))
            .size(13)
            .font(MONO)
            .style(style)
            .width(Fill),
    ]
    .align_y(Alignment::Center)
    .spacing(8)
    .into()
}

fn text_input_style(
    t: &Theme,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let dark = is_dark(t);
    let focused = matches!(status, iced::widget::text_input::Status::Focused { .. });
    let accent = if dark { c("#3DD68C") } else { c("#18A862") };
    let border_def = if dark { c("#232A34") } else { c("#DDE2E8") };
    iced::widget::text_input::Style {
        background: Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") }),
        border: Border {
            radius: 7.0.into(),
            width: 1.0,
            color: if focused { accent } else { border_def },
        },
        icon: Color::TRANSPARENT,
        placeholder: if dark { c("#5C636F") } else { c("#A0A7B1") },
        value: if dark { c("#E8EBF0") } else { c("#161A20") },
        selection: Color { r: 0.239, g: 0.839, b: 0.549, a: 0.25 },
    }
}

fn text_input_error_style(
    t: &Theme,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let dark = is_dark(t);
    let focused = matches!(status, iced::widget::text_input::Status::Focused { .. });
    let danger = if dark { c("#E5604D") } else { c("#CC3A28") };
    let danger_dim = if dark { c("#6B2020") } else { c("#EDA8A0") };
    iced::widget::text_input::Style {
        background: Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") }),
        border: Border {
            radius: 7.0.into(),
            width: 1.5,
            color: if focused { danger } else { danger_dim },
        },
        icon: Color::TRANSPARENT,
        placeholder: if dark { c("#5C636F") } else { c("#A0A7B1") },
        value: danger,
        selection: Color { r: 0.898, g: 0.376, b: 0.302, a: 0.25 },
    }
}

pub fn text_editor_style(
    t: &Theme,
    _: iced::widget::text_editor::Status,
) -> iced::widget::text_editor::Style {
    let dark = is_dark(t);
    iced::widget::text_editor::Style {
        background: Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") }),
        border: Border {
            color: if dark { c("#232A34") } else { c("#DDE2E8") },
            width: 1.0,
            radius: 7.0.into(),
        },
        placeholder: if dark { c("#5C636F") } else { c("#A0A7B1") },
        value: if dark { c("#E8EBF0") } else { c("#161A20") },
        selection: Color { r: 0.239, g: 0.839, b: 0.549, a: 0.25 },
    }
}
