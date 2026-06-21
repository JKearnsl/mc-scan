use iced::widget::space::Space;
use iced::widget::{button, column, container, row, text};
use iced::Length::Fixed;
use iced::{Element, Fill, Theme};

use crate::app::{McScan, Message};
use crate::components::ui::{dialog, labeled_input, section_label};
use crate::styles::{button_primary, button_secondary, c, is_dark, SANS_SEMIBOLD};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let is_dark_theme = app.is_dark;

    let body = column![
        Space::new().height(16),
        section_label("ТЕМА"),
        Space::new().height(8),
        row![
            button(container(text("Тёмная").size(13).font(SANS_SEMIBOLD)).center(Fill))
                .style(if is_dark_theme { button_primary } else { button_secondary })
                .on_press(Message::SetTheme(true))
                .width(Fill)
                .height(Fixed(36.0)),
            Space::new().width(8),
            button(container(text("Светлая").size(13).font(SANS_SEMIBOLD)).center(Fill))
                .style(if !is_dark_theme { button_primary } else { button_secondary })
                .on_press(Message::SetTheme(false))
                .width(Fill)
                .height(Fixed(36.0)),
        ],
        Space::new().height(16),
        section_label("ПОРТЫ"),
        Space::new().height(8),
        labeled_input("Java",       &app.settings.java_ports,    "25565", Message::JavaPortsChanged,    app.settings.java_ports_error),
        Space::new().height(6),
        labeled_input("Bedrock",    &app.settings.bedrock_ports,  "19132", Message::BedrockPortsChanged, app.settings.bedrock_ports_error),
        Space::new().height(16),
        section_label("ПАРАМЕТРЫ"),
        Space::new().height(8),
        labeled_input("Потоки",     &app.settings.concurrency,    "1024",  Message::ConcurrencyChanged,  false),
        Space::new().height(6),
        labeled_input("Таймаут мс", &app.settings.timeout_ms,     "1500",  Message::TimeoutChanged,      false),
        Space::new().height(20),
        button(
            container(
                text("Готово").size(15).font(SANS_SEMIBOLD)
                    .style(|t: &Theme| iced::widget::text::Style {
                        color: Some(if is_dark(t) { c("#08110B") } else { c("#FFFFFF") }),
                    }),
            )
            .center(Fill),
        )
        .on_press(Message::CloseModal)
        .style(button_primary)
        .width(Fill)
        .height(Fixed(44.0)),
    ];

    dialog("Настройки", Message::CloseModal, 380.0, body.into())
}
