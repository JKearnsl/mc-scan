use iced::widget::container::Style as ContainerStyle;
use iced::widget::{container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Padding, Theme};

use crate::styles::{c, is_dark, MONO};

pub fn status<'a, M: Clone + 'a>(label: String) -> Element<'a, M> {
    let inner = row![
        text("●").size(9).font(MONO).style(|t: &Theme| text::Style {
            color: Some(if is_dark(t) { c("#3DD68C") } else { c("#18A862") }),
        }),
        text(label).size(13).font(MONO).style(|t: &Theme| text::Style {
            color: Some(if is_dark(t) { c("#9FE9C4") } else { c("#0B6040") }),
        }),
    ]
    .align_y(Alignment::Center)
    .spacing(8);

    container(inner)
        .style(|t: &Theme| {
            let (bg_a, brd_a) = if is_dark(t) { (0.12f32, 0.25f32) } else { (0.08, 0.20) };
            let (r, g, b) =
                if is_dark(t) { (0.239f32, 0.839, 0.549) } else { (0.094, 0.659, 0.384) };
            ContainerStyle {
                background: Some(Background::Color(Color { r, g, b, a: bg_a })),
                border: Border { color: Color { r, g, b, a: brd_a }, width: 1.0, radius: 8.0.into() },
                ..Default::default()
            }
        })
        .padding(Padding { top: 5.0, right: 11.0, bottom: 5.0, left: 11.0 })
        .into()
}

pub fn chip<'a, M: Clone + 'a>(label: String) -> Element<'a, M> {
    container(text(label).size(11).font(MONO).style(|t: &Theme| text::Style {
        color: Some(if is_dark(t) { c("#A2ABBA") } else { c("#3A4049") }),
    }))
    .style(|t: &Theme| ContainerStyle {
        background: Some(Background::Color(if is_dark(t) { c("#1F2630") } else { c("#EEF0F3") })),
        border: Border {
            color: if is_dark(t) { c("#2A3240") } else { c("#DDE2E8") },
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
    .padding(Padding::from([2, 6]))
    .into()
}
