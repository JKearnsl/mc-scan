use iced::widget::space::Space;
use iced::widget::{column, row, text};
use iced::{Element, Theme};

use crate::app::{McScan, Message};
use crate::components::ui::{btn, dialog, textarea, BtnVariant};
use crate::styles::{c, is_dark, SANS};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let tr = app.tr();

    let body = column![
        Space::new().height(4),
        text(tr.add_ranges_hint)
            .size(11)
            .font(SANS)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
            }),
        Space::new().height(12),
        textarea(&app.ranges_editor, Message::RangesEditorAction, 160.0),
        Space::new().height(16),
        row![
            btn(BtnVariant::Danger(tr.cancel), Message::CloseModal),
            Space::new().width(10),
            btn(BtnVariant::Primary(tr.add), Message::ConfirmAddRanges),
        ],
    ];

    dialog(tr.add_ranges_title, Message::CloseModal, Message::NoOp, 460.0, body.into())
}
