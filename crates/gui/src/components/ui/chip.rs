use iced::Length::Fixed;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::{Space, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Padding, Theme};

use crate::styles::{MONO, SANS, c, is_dark};

/// Compact monospace tag.
pub fn chip<'a, M: Clone + 'a>(label: String) -> Element<'a, M> {
    container(
        text(label)
            .size(11)
            .font(MONO)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) {
                    c("#A2ABBA")
                } else {
                    c("#3A4049")
                }),
            }),
    )
    .style(|t: &Theme| ContainerStyle {
        background: Some(Background::Color(if is_dark(t) {
            c("#1F2630")
        } else {
            c("#EEF0F3")
        })),
        border: Border {
            color: if is_dark(t) {
                c("#2A3240")
            } else {
                c("#DDE2E8")
            },
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
    .padding(Padding::from([2, 6]))
    .into()
}

/// Rounded tag with a leading colored dot. The dot color is caller-supplied.
pub fn chip_dot<'a, M: 'a>(label: String, dot: Color) -> Element<'a, M> {
    container(
        row![
            container(Space::new())
                .width(Fixed(7.0))
                .height(Fixed(7.0))
                .style(move |_: &Theme| ContainerStyle {
                    background: Some(Background::Color(dot)),
                    border: Border {
                        radius: 3.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            text(label)
                .size(13)
                .font(SANS)
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) {
                        c("#A2ABBA")
                    } else {
                        c("#3A4049")
                    }),
                }),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .style(|t: &Theme| ContainerStyle {
        background: Some(Background::Color(if is_dark(t) {
            c("#191E27")
        } else {
            c("#EEF0F3")
        })),
        border: Border {
            color: if is_dark(t) {
                c("#262E3C")
            } else {
                c("#DDE2E8")
            },
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .padding(Padding {
        top: 6.0,
        right: 10.0,
        bottom: 6.0,
        left: 8.0,
    })
    .into()
}
