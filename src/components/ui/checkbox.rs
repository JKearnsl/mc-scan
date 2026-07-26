use crate::app::Message;
use iced::widget::space::Space;
use iced::widget::toggler::Status;
use iced::widget::{row, text, toggler};
use iced::{Alignment, Element, Fill};
use iced::Theme;

use crate::styles::{c, is_dark, SANS};


pub fn checkbox(
    label: &'static str,
    value: bool,
    on_toggle: fn(bool) -> Message,
) -> Element<'static, Message> {
    row![
        text(label)
            .size(13)
            .font(SANS)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#C3CAD4") } else { c("#3A4049") }),
            }),
        Space::new().width(Fill),
        toggler(value)
            .on_toggle(on_toggle)
            .size(20)
            .style(|t: &Theme, status: Status| {
                let mut style = toggler::default(t, status);
                style.foreground = t.extended_palette().background.base.color.into();
                style
            }),
    ]
        .align_y(Alignment::Center)
        .width(Fill)
        .into()
}
