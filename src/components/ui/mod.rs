mod dialog;
mod input;

pub use dialog::dialog;
pub use input::{labeled_input, text_editor_style};

use iced::{Element, Theme};
use crate::styles::{c, is_dark, SANS_SEMIBOLD};

pub fn section_label<'a, M: Clone + 'a>(label: &'a str) -> Element<'a, M> {
    use iced::widget::text;
    text(label)
        .size(11)
        .font(SANS_SEMIBOLD)
        .style(|t: &Theme| iced::widget::text::Style {
            color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
        })
        .into()
}
