use iced::Length::Fixed;
use iced::widget::{progress_bar as iced_progress_bar};
use iced::{Border, Color, Element, Fill, Theme};

use crate::styles::{c, is_dark};

pub fn progress_bar<'a, M: 'a>(ratio: f32) -> Element<'a, M> {
    iced_progress_bar(0.0..=1.0, ratio)
        .style(style)
        .girth(Fixed(4.0))
        .length(Fill)
        .into()
}

fn style(t: &Theme) -> iced::widget::progress_bar::Style {
    iced::widget::progress_bar::Style {
        background: iced::Background::Color(if is_dark(t) {
            c("#1A1F27")
        } else {
            c("#E1E5EA")
        }),
        bar: iced::Background::Color(if is_dark(t) {
            c("#3DD68C")
        } else {
            c("#18A862")
        }),
        border: Border {
            radius: 2.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
    }
}
