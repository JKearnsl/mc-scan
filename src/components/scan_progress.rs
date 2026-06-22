use iced::widget::space::Space;
use iced::widget::{column, progress_bar, row, text};
use iced::Length::Fixed;
use iced::{Alignment, Border, Color, Element, Fill, Theme};

use crate::app::{McScan, Message};
use crate::styles::{c, is_dark, MONO};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let ratio = app.scanned_count as f32 / app.total_targets as f32;
    let pct = (ratio * 100.0) as u32;
    let range_str = app.address_list.values().first()
        .map(|r| r.to_string())
        .unwrap_or_else(|| "…".to_string());
    let scanned = app.scanned_count;
    let total = app.total_targets;

    column![
        progress_bar(0.0..=1.0, ratio)
            .style(progress_bar_style)
            .girth(Fixed(4.0))
            .length(Fill),
        Space::new().height(Fixed(9.0)),
        row![
            text(format!("Сканирование {}", range_str)).size(12).font(MONO)
                .style(|t: &Theme| iced::widget::text::Style {
                    color: Some(if is_dark(t) { c("#6B7480") } else { c("#8A929E") }),
                }),
            Space::new().width(Fill),
            text(format!("{}% · {} / {}", pct, scanned, total)).size(12).font(MONO)
                .style(|t: &Theme| iced::widget::text::Style {
                    color: Some(if is_dark(t) { c("#8C95A3") } else { c("#6B7480") }),
                }),
        ]
        .align_y(Alignment::Center),
    ]
    .into()
}

fn progress_bar_style(t: &Theme) -> iced::widget::progress_bar::Style {
    progress_bar::Style {
        background: iced::Background::Color(if is_dark(t) { c("#1A1F27") } else { c("#E1E5EA") }),
        bar: iced::Background::Color(if is_dark(t) { c("#3DD68C") } else { c("#18A862") }),
        border: Border { radius: 2.0.into(), width: 0.0, color: Color::TRANSPARENT },
    }
}
