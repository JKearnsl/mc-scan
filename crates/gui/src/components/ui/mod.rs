mod badge;
mod button;
mod checkbox;
mod dialog;
pub mod icons;
mod input;
mod scrollbar;
mod textarea;
mod wrap;

pub use badge::{chip, status};
pub use button::{BtnVariant, btn, button_danger, button_primary};
pub use checkbox::checkbox;
pub use dialog::dialog;
pub use input::labeled_input;
pub use scrollbar::scrollbar;
pub use textarea::textarea;
pub use wrap::wrap;

use iced::Length::Fixed;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::{Space, container};
use iced::{Background, Element, Fill, Theme};

use crate::styles::{SANS_SEMIBOLD, c, is_dark};

pub fn section_label<'a, M: Clone + 'a>(label: &'a str) -> Element<'a, M> {
    use iced::widget::text;
    text(label)
        .size(11)
        .font(SANS_SEMIBOLD)
        .style(|t: &Theme| text::Style {
            color: Some(if is_dark(t) {
                c("#5C636F")
            } else {
                c("#A0A7B1")
            }),
        })
        .into()
}

pub fn divider<'a, M: Clone + 'a>() -> Element<'a, M> {
    container(Space::new())
        .style(|t: &Theme| ContainerStyle {
            background: Some(Background::Color(if is_dark(t) {
                c("#1A1F27")
            } else {
                c("#E1E5EA")
            })),
            ..Default::default()
        })
        .width(Fill)
        .height(Fixed(1.0))
        .into()
}
