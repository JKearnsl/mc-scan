use iced::widget::text;
use iced::{Element, Theme};

use crate::styles::{SANS_SEMIBOLD, c, is_dark};

/// Primary title, single line.
pub fn heading<'a, M: 'a>(label: impl text::IntoFragment<'a>) -> Element<'a, M> {
    text(label)
        .size(17)
        .font(SANS_SEMIBOLD)
        .wrapping(text::Wrapping::None)
        .style(|t: &Theme| text::Style {
            color: Some(if is_dark(t) {
                c("#E8EBF0")
            } else {
                c("#161A20")
            }),
        })
        .into()
}
