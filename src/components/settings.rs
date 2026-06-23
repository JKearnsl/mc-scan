use iced::widget::space::Space;
use iced::widget::{column, row};
use iced::{Element};

use crate::app::{McScan, Message};
use crate::components::ui::{btn, dialog, labeled_input, section_label, BtnVariant};
use crate::i18n::Language;

pub fn render(app: &McScan) -> Element<'_, Message> {
    let tr = app.tr();
    let is_dark = app.is_dark;
    let lang = app.language;

    let body = column![
        Space::new().height(16),
        section_label(tr.theme),
        Space::new().height(8),
        row![
            btn(
                if is_dark { BtnVariant::Primary(tr.dark) } else { BtnVariant::Secondary(tr.dark) },
                Message::SetTheme(true),
            ),
            Space::new().width(8),
            btn(
                if !is_dark { BtnVariant::Primary(tr.light) } else { BtnVariant::Secondary(tr.light) },
                Message::SetTheme(false),
            ),
        ],
        Space::new().height(16),
        section_label(tr.language),
        Space::new().height(8),
        row![
            btn(
                if lang == Language::English { BtnVariant::Primary("EN") } else { BtnVariant::Secondary("EN") },
                Message::SetLanguage(Language::English),
            ),
            Space::new().width(8),
            btn(
                if lang == Language::Russian { BtnVariant::Primary("RU") } else { BtnVariant::Secondary("RU") },
                Message::SetLanguage(Language::Russian),
            ),
            Space::new().width(8),
            btn(
                if lang == Language::Chinese { BtnVariant::Primary("中文") } else { BtnVariant::Secondary("中文") },
                Message::SetLanguage(Language::Chinese),
            ),
            Space::new().width(8),
            btn(
                if lang == Language::Japanese { BtnVariant::Primary("日本語") } else { BtnVariant::Secondary("日本語") },
                Message::SetLanguage(Language::Japanese),
            ),
        ],
        Space::new().height(16),
        section_label(tr.ports),
        Space::new().height(8),
        labeled_input("Java",        &app.settings.java_ports,    "25565", Message::JavaPortsChanged,    app.settings.java_ports_error),
        Space::new().height(6),
        labeled_input("Bedrock",     &app.settings.bedrock_ports,  "19132", Message::BedrockPortsChanged, app.settings.bedrock_ports_error),
        Space::new().height(16),
        section_label(tr.parameters),
        Space::new().height(8),
        labeled_input(tr.threads,    &app.settings.concurrency,    "1024",  Message::ConcurrencyChanged,  false),
        Space::new().height(6),
        labeled_input(tr.timeout_ms, &app.settings.timeout_ms,     "1500",  Message::TimeoutChanged,      false),
        Space::new().height(20),
        btn(BtnVariant::Primary(tr.done), Message::CloseModal),
    ];

    dialog(tr.settings, Message::CloseModal, 380.0, body.into())
}
