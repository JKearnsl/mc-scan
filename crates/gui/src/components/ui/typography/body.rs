use iced::widget::text;
use iced::{Element, Theme};

use crate::styles::{SANS, c, is_dark};

/// Regular secondary paragraph text.
pub fn body<'a, M: 'a>(label: impl text::IntoFragment<'a>) -> Element<'a, M> {
    text(label)
        .size(13)
        .font(SANS)
        .style(|t: &Theme| text::Style {
            color: Some(if is_dark(t) {
                c("#A2ABBA")
            } else {
                c("#4A5260")
            }),
        })
        .into()
}
