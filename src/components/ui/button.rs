use iced::widget::{button, container, text};
use iced::Length::Fixed;
use iced::{Element, Fill};

use crate::styles::{button_danger, button_primary, SANS_SEMIBOLD};

pub enum BtnVariant {
    Primary,
    Danger,
}

pub fn btn<'a, M: Clone + 'a>(
    label: &'a str,
    on_press: M,
    variant: BtnVariant,
) -> Element<'a, M> {
    let style: fn(&iced::Theme, button::Status) -> button::Style =
        match variant {
            BtnVariant::Primary => button_primary,
            BtnVariant::Danger => button_danger,
        };

    button(container(text(label).size(14).font(SANS_SEMIBOLD)).center(Fill))
        .on_press(on_press)
        .style(style)
        .width(Fill)
        .height(Fixed(44.0))
        .into()
}
