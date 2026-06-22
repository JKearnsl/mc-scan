use iced::widget::space::Space;
use iced::widget::{column, container, progress_bar, row, text};
use iced::Length::Fixed;
use iced::{Alignment, Border, Color, Element, Fill, Padding, Theme};

use crate::app::{McScan, Message};
use crate::components::ui::{divider, status_badge};
use crate::styles::{c, is_dark, MONO, MONO_SEMIBOLD, SANS};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let header = container(header_col(app))
        .style(header_style_fn)
        .padding(Padding { top: 18.0, right: 20.0, bottom: 16.0, left: 20.0 })
        .width(Fill);

    let results = app.results.view().map(Message::ResultsList);

    column![header, divider(), results].width(Fill).height(Fill).into()
}

fn header_col(app: &McScan) -> Element<'_, Message> {
    let mut col = column![title_row(app)].spacing(0);

    if app.is_scanning && app.total_targets > 0 {
        col = col
            .push(Space::new().height(Fixed(14.0)))
            .push(scan_progress(app));
    }

    col.into()
}

fn title_row(app: &McScan) -> Element<'_, Message> {
    let found = app.results.count();

    let title = row![
        text("mc-scan").size(18).font(MONO_SEMIBOLD)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#E8EBF0") } else { c("#161A20") }),
            }),
        container(
            text("Сканер Minecraft-серверов").size(13).font(SANS)
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) { c("#6B7480") } else { c("#8A929E") }),
                })
        ).padding(Padding { top: 3.0, ..Default::default() }),
    ]
    .align_y(Alignment::Center)
    .spacing(12);

    let mut r = row![title, Space::new().width(Fill)].align_y(Alignment::Center);

    if found > 0 {
        r = r.push(status_badge(format!("{} найдено", found)));
    }

    r.into()
}

fn header_style_fn(t: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(if is_dark(t) { c("#0E1116") } else { c("#FFFFFF") })),
        ..Default::default()
    }
}

fn progress_bar_style_fn(t: &Theme) -> iced::widget::progress_bar::Style {
    iced::widget::progress_bar::Style {
        background: iced::Background::Color(if is_dark(t) { c("#1A1F27") } else { c("#E1E5EA") }),
        bar: iced::Background::Color(if is_dark(t) { c("#3DD68C") } else { c("#18A862") }),
        border: Border { radius: 2.0.into(), width: 0.0, color: Color::TRANSPARENT },
    }
}

fn scan_progress(app: &McScan) -> Element<'_, Message> {
    let ratio = app.scanned_count as f32 / app.total_targets as f32;
    let pct = (ratio * 100.0) as u32;
    let range_str = app.address_list.values().first()
        .map(|r| r.to_string())
        .unwrap_or_else(|| "…".to_string());
    let scanned = app.scanned_count;
    let total = app.total_targets;

    column![
        progress_bar(0.0..=1.0, ratio)
            .style(progress_bar_style_fn)
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
