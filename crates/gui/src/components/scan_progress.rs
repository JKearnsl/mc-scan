use iced::Length::Fixed;
use iced::widget::space::Space;
use iced::widget::{column, row, text};
use iced::{Alignment, Element, Fill, Theme};

use crate::app::{McScan, Message};
use crate::components::ui::progress_bar;
use crate::styles::{MONO, c, is_dark};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let tr = app.tr();
    let ratio = app.scanned_count as f32 / app.total_targets as f32;
    let pct = (ratio * 100.0) as u32;
    let ranges = app.address_list.values();
    let range_str = match ranges.len() {
        0 => "…".to_string(),
        1 => ranges[0].to_string(),
        n => format!("{} (+{})", ranges[0], n - 1),
    };
    let scanned = app.scanned_count;
    let total = app.total_targets;

    column![
        progress_bar(ratio),
        Space::new().height(Fixed(9.0)),
        row![
            text(format!("{} {}", tr.scanning, range_str))
                .size(12)
                .font(MONO)
                .style(|t: &Theme| iced::widget::text::Style {
                    color: Some(if is_dark(t) {
                        c("#6B7480")
                    } else {
                        c("#8A929E")
                    }),
                }),
            Space::new().width(Fill),
            text(format!("{}% · {} / {}", pct, scanned, total))
                .size(12)
                .font(MONO)
                .style(|t: &Theme| iced::widget::text::Style {
                    color: Some(if is_dark(t) {
                        c("#8C95A3")
                    } else {
                        c("#6B7480")
                    }),
                }),
        ]
        .align_y(Alignment::Center),
    ]
    .into()
}
