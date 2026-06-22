mod badge;
mod button;
mod dialog;
mod input;
mod scrollbar;
mod textarea;

pub use badge::{chip, status};
pub use button::{btn, BtnVariant};
pub use dialog::dialog;
pub use input::labeled_input;
pub use scrollbar::scrollbar;
pub use textarea::textarea;

use iced::widget::container::Style as ContainerStyle;
use iced::widget::{container, Space};
use iced::Length::Fixed;
use iced::{Background, Element, Fill, Theme};

use crate::styles::{c, is_dark, SANS_SEMIBOLD};

pub fn section_label<'a, M: Clone + 'a>(label: &'a str) -> Element<'a, M> {
    use iced::widget::text;
    text(label)
        .size(11)
        .font(SANS_SEMIBOLD)
        .style(|t: &Theme| text::Style {
            color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
        })
        .into()
}

pub fn divider<'a, M: Clone + 'a>() -> Element<'a, M> {
    container(Space::new())
        .style(|t: &Theme| ContainerStyle {
            background: Some(Background::Color(if is_dark(t) { c("#1A1F27") } else { c("#E1E5EA") })),
            ..Default::default()
        })
        .width(Fill)
        .height(Fixed(1.0))
        .into()
}
