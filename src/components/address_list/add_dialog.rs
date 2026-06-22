use iced::widget::space::Space;
use iced::widget::{column, row, text};
use iced::{Element, Theme};

use crate::app::{McScan, Message};
use crate::components::ui::{btn, dialog, textarea, BtnVariant};
use crate::styles::{c, is_dark, SANS};

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
        textarea(&app.ranges_editor, Message::RangesEditorAction, 160.0),
        Space::new().height(16),
        row![
            btn("Отмена", Message::CloseModal, BtnVariant::Danger),
            Space::new().width(10),
            btn("Добавить", Message::ConfirmAddRanges, BtnVariant::Primary),
        ],
    ];

    dialog("Добавить диапазоны", Message::CloseModal, 460.0, body.into())
}
