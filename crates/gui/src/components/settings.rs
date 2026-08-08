use iced::Element;
use iced::widget::space::Space;
use iced::widget::{column, row};

use crate::app::{McScan, Message};
use crate::components::ui::{BtnVariant, btn, caption, checkbox, dialog, labeled_input};
use crate::config::ThemePref;
use crate::i18n::Language;

pub fn render(app: &McScan) -> Element<'_, Message> {
    let tr = app.tr();
    let theme = app.theme_pref;
    let lang = app.language;

    let theme_btn = |label: &'static str, pref: ThemePref| {
        let variant = if theme == pref {
            BtnVariant::Primary(label)
        } else {
            BtnVariant::Secondary(label)
        };
        btn(variant, Message::SetThemePref(pref))
    };

    let body = column![
        Space::new().height(16),
        caption(tr.theme, 11),
        Space::new().height(8),
        row![
            theme_btn(tr.system, ThemePref::System),
            Space::new().width(8),
            theme_btn(tr.dark, ThemePref::Dark),
            Space::new().width(8),
            theme_btn(tr.light, ThemePref::Light),
        ],
        Space::new().height(16),
        caption(tr.language, 11),
        Space::new().height(8),
        row![
            btn(
                if lang == Language::English {
                    BtnVariant::Primary("EN")
                } else {
                    BtnVariant::Secondary("EN")
                },
                Message::SetLanguage(Language::English),
            ),
            Space::new().width(8),
            btn(
                if lang == Language::Russian {
                    BtnVariant::Primary("RU")
                } else {
                    BtnVariant::Secondary("RU")
                },
                Message::SetLanguage(Language::Russian),
            ),
            Space::new().width(8),
            btn(
                if lang == Language::Chinese {
                    BtnVariant::Primary("中文")
                } else {
                    BtnVariant::Secondary("中文")
                },
                Message::SetLanguage(Language::Chinese),
            ),
            Space::new().width(8),
            btn(
                if lang == Language::Japanese {
                    BtnVariant::Primary("日本語")
                } else {
                    BtnVariant::Secondary("日本語")
                },
                Message::SetLanguage(Language::Japanese),
            ),
        ],
        Space::new().height(16),
        caption(tr.ports, 11),
        Space::new().height(8),
        labeled_input(
            "Java",
            &app.settings.java_ports,
            "25565",
            Message::JavaPortsChanged,
            app.settings.java_ports_error
        ),
        Space::new().height(6),
        labeled_input(
            "Bedrock",
            &app.settings.bedrock_ports,
            "19132",
            Message::BedrockPortsChanged,
            app.settings.bedrock_ports_error
        ),
        Space::new().height(16),
        caption(tr.parameters, 11),
        Space::new().height(8),
        labeled_input(
            tr.threads,
            &app.settings.concurrency,
            "1024",
            Message::ConcurrencyChanged,
            false
        ),
        Space::new().height(6),
        labeled_input(
            tr.timeout_ms,
            &app.settings.timeout_ms,
            "1500",
            Message::TimeoutChanged,
            false
        ),
        Space::new().height(16),
        caption(tr.enrichment, 11),
        Space::new().height(8),
        checkbox(
            tr.query_label,
            app.settings.query_enabled,
            Message::ToggleQuery
        ),
        Space::new().height(8),
        checkbox(
            tr.online_mode_label,
            app.settings.online_mode_check,
            Message::ToggleOnlineModeCheck
        ),
    ];

    dialog(
        tr.settings,
        Message::CloseModal,
        Message::NoOp,
        380.0,
        540.0,
        body.into(),
    )
}
