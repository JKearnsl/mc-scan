use iced::widget::space::Space;
use iced::widget::{column, row};
use iced::{Element};

use crate::app::{McScan, Message};
use crate::components::ui::{btn, dialog, labeled_input, section_label, BtnVariant};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let is_dark_theme = app.is_dark;

    let body = column![
        Space::new().height(16),
        section_label("ТЕМА"),
        Space::new().height(8),
        row![
            btn(
                if is_dark_theme { BtnVariant::Primary("Тёмная") } else { BtnVariant::Secondary("Тёмная") },
                Message::SetTheme(true),
            ),
            Space::new().width(8),
            btn(
                if !is_dark_theme { BtnVariant::Primary("Светлая") } else { BtnVariant::Secondary("Светлая") },
                Message::SetTheme(false),
            ),
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
        btn(BtnVariant::Primary("Готово"), Message::CloseModal),
    ];

    dialog("Настройки", Message::CloseModal, 380.0, body.into())
}
