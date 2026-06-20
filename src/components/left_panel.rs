use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{column, container, progress_bar, row, text};
use iced::Length::Fixed;
use iced::{Alignment, Background, Border, Color, Element, Fill, Padding, Theme};

use crate::app::{McScan, Message};
use crate::styles::{c, is_dark, MONO, MONO_SEMIBOLD, SANS};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let header = container(header_col(app))
        .style(header_style_fn)
        .padding(Padding { top: 18.0, right: 20.0, bottom: 16.0, left: 20.0 })
        .width(Fill);

    let divider = container(Space::new().height(Fixed(0.0)))
        .style(|t: &Theme| ContainerStyle {
            background: Some(Background::Color(if is_dark(t) { c("#1A1F27") } else { c("#E1E5EA") })),
            ..Default::default()
        })
        .width(Fill)
        .height(Fixed(1.0));

    let results = app.results.view().map(Message::ResultsList);

    column![header, divider, results].width(Fill).height(Fill).into()
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

    let badge = container(
        row![
            text("●").size(9).font(MONO)
                .style(|t: &Theme| iced::widget::text::Style {
                    color: Some(if is_dark(t) { c("#3DD68C") } else { c("#18A862") }),
                }),
            text(format!("{} найдено", found)).size(13).font(MONO)
                .style(|t: &Theme| iced::widget::text::Style {
                    color: Some(if is_dark(t) { c("#9FE9C4") } else { c("#0B6040") }),
                }),
        ]
        .align_y(Alignment::Center)
        .spacing(8),
    )
    .style(|t: &Theme| {
        let (bg_a, brd_a) = if is_dark(t) { (0.12f32, 0.25f32) } else { (0.08, 0.20) };
        let (r, g, b) = if is_dark(t) { (0.239f32, 0.839, 0.549) } else { (0.094, 0.659, 0.384) };
        ContainerStyle {
            background: Some(Background::Color(Color { r, g, b, a: bg_a })),
            border: Border { color: Color { r, g, b, a: brd_a }, width: 1.0, radius: 8.0.into() },
            ..Default::default()
        }
    })
    .padding(Padding { top: 5.0, right: 11.0, bottom: 5.0, left: 11.0 });

    row![
        row![
            text("mc-scan").size(18).font(MONO_SEMIBOLD)
                .style(|t: &Theme| iced::widget::text::Style {
                    color: Some(if is_dark(t) { c("#E8EBF0") } else { c("#161A20") }),
                }),
            text("сканер Minecraft-серверов").size(13).font(SANS)
                .style(|t: &Theme| iced::widget::text::Style {
                    color: Some(if is_dark(t) { c("#6B7480") } else { c("#8A929E") }),
                }),
        ]
        .align_y(Alignment::Center)
        .spacing(12),
        Space::new().width(Fill),
        badge,
    ]
    .align_y(Alignment::Center)
    .into()
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
