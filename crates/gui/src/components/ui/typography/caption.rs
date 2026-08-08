use iced::widget::text;
use iced::{Element, Theme};

use crate::styles::{SANS_SEMIBOLD, c, is_dark};

/// Small semibold overline used above fields, cells and panel sections.
pub fn caption<'a, M: 'a>(label: impl text::IntoFragment<'a>, size: u16) -> Element<'a, M> {
    text(label)
        .size(f32::from(size))
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
