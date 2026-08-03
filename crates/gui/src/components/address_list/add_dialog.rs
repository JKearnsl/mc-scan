use iced::widget::space::Space;
use iced::widget::{column, row, text};
use iced::{Element, Theme};

use crate::app::{McScan, Message};
use crate::components::ui::{BtnVariant, btn, dialog, textarea};
use crate::styles::{SANS, c, is_dark};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let tr = app.tr();

    let body = column![
        Space::new().height(4),
        text(tr.add_ranges_hint)
            .size(11)
            .font(SANS)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) {
                    c("#5C636F")
                } else {
                    c("#A0A7B1")
                }),
            }),
        Space::new().height(12),
        textarea(&app.ranges_editor, Message::RangesEditorAction, 160.0),
        rejected_notice(app),
        Space::new().height(16),
        row![
            btn(BtnVariant::Danger(tr.cancel), Message::CloseModal),
            Space::new().width(10),
            btn(BtnVariant::Primary(tr.add), Message::ConfirmAddRanges),
        ],
    ];

    dialog(
        tr.add_ranges_title,
        Message::CloseModal,
        Message::NoOp,
        460.0,
        540.0,
        body.into(),
    )
}

/// Warns about input lines the last confirm couldn't parse (they are left in the
/// editor for correction). Collapses to nothing when there is nothing to report.
fn rejected_notice(app: &McScan) -> Element<'_, Message> {
    if app.rejected_ranges == 0 {
        return Space::new().height(0).into();
    }
    let msg = format!(
        "\u{26A0} {} {}",
        app.rejected_ranges,
        app.tr().ranges_rejected
    );
    column![
        Space::new().height(8),
        text(msg)
            .size(11)
            .font(SANS)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) {
                    c("#E5A24D")
                } else {
                    c("#B36A16")
                }),
            }),
    ]
    .into()
}
