use iced::widget::text;
use iced::{Element, Theme};

use crate::styles::{SANS_SEMIBOLD, c, is_dark};

pub fn section_label<'a, M: Clone + 'a>(label: &'a str) -> Element<'a, M> {
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
