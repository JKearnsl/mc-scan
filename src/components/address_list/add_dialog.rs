use iced::widget::space::Space;
use iced::widget::{button, column, container, row, text, text_editor};
use iced::Length::Fixed;
use iced::{Element, Fill, Padding, Theme};

use crate::app::{McScan, Message};
use crate::components::ui::{dialog, text_editor_style};
use crate::styles::{button_danger, button_primary, c, is_dark, MONO, SANS, SANS_SEMIBOLD};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let body = column![
        Space::new().height(4),
        text("CIDR (10.0.0.0/8) · диапазон (1.2.3.4-1.2.3.100) · одиночный IP")
            .size(11)
            .font(SANS)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
            }),
        Space::new().height(12),
        text_editor(&app.ranges_editor)
            .on_action(Message::RangesEditorAction)
            .height(Fixed(160.0))
            .style(text_editor_style)
            .font(MONO)
            .size(13)
            .padding(Padding::from([8, 10])),
        Space::new().height(16),
        row![
            button(
                container(
                    text("Отмена").size(14).font(SANS_SEMIBOLD)
                        .style(|_: &Theme| text::Style { color: Some(c("#FFFFFF")) }),
                )
                .center(Fill),
            )
            .on_press(Message::CloseModal)
            .style(button_danger)
            .width(Fill)
            .height(Fixed(44.0)),
            Space::new().width(10),
            button(
                container(
                    text("Добавить").size(14).font(SANS_SEMIBOLD)
                        .style(|t: &Theme| text::Style {
                            color: Some(if is_dark(t) { c("#08110B") } else { c("#FFFFFF") }),
                        }),
                )
                .center(Fill),
            )
            .on_press(Message::ConfirmAddRanges)
            .style(button_primary)
            .width(Fill)
            .height(Fixed(44.0)),
        ],
    ];

    dialog("Добавить диапазоны", Message::CloseModal, 460.0, body.into())
}
