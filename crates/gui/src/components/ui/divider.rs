use iced::Length::Fixed;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::{Space, container};
use iced::{Background, Element, Fill, Theme};

use crate::styles::{c, is_dark};

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
