use iced::widget::container::Style as ContainerStyle;
use iced::widget::{Space, column, container, text};
use iced::{Background, Border, Color, Element, Fill, Padding, Theme};

use super::typography::caption;
use crate::styles::{MONO_SEMIBOLD, SANS_SEMIBOLD, c, is_dark};

/// Bordered box with a caption above a single value.
pub fn cell<'a, M: 'a>(label: &'a str, value: String, mono: bool) -> Element<'a, M> {
    build(
        label,
        text(value)
            .size(14)
            .wrapping(text::Wrapping::None)
            .font(if mono { MONO_SEMIBOLD } else { SANS_SEMIBOLD })
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) {
                    c("#E8EBF0")
                } else {
                    c("#161A20")
                }),
            })
            .into(),
    )
}

/// Same as [`cell`], but the value is rendered in an explicit accent color.
pub fn cell_colored<'a, M: 'a>(
    label: &'a str,
    value: String,
    value_color: Color,
) -> Element<'a, M> {
    build(
        label,
        text(value)
            .size(14)
            .wrapping(text::Wrapping::None)
            .font(MONO_SEMIBOLD)
            .style(move |_: &Theme| text::Style {
                color: Some(value_color),
            })
            .into(),
    )
}

fn build<'a, M: 'a>(label: &'a str, value: Element<'a, M>) -> Element<'a, M> {
    container(
        column![caption(label, 10), Space::new().height(5), value]
            .width(Fill)
            .clip(true),
    )
    .style(card_style)
    .padding(Padding::from([10, 12]))
    .width(Fill)
    .into()
}

fn card_style(t: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Some(Background::Color(if is_dark(t) {
            c("#131821")
        } else {
            c("#F6F8FA")
        })),
        border: Border {
            color: if is_dark(t) {
                c("#1E2530")
            } else {
                c("#E5E9EF")
            },
            width: 1.0,
            radius: 9.0.into(),
        },
        ..Default::default()
    }
}
