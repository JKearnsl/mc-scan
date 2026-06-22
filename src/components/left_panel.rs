use iced::widget::space::Space;
use iced::widget::{column, container, row, text};
use iced::Length::Fixed;
use iced::{Alignment, Element, Fill, Padding, Theme};

use crate::app::{McScan, Message};
use crate::components::{scan_progress, ui::{divider, status_badge}};
use crate::styles::{c, is_dark, MONO_SEMIBOLD, SANS};

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
            .push(scan_progress::render(app));
    }

    col.into()
}

fn title_row(app: &McScan) -> Element<'_, Message> {
    let found = app.results.count();

    let title = row![
        text(env!("CARGO_PKG_NAME")).size(18).font(MONO_SEMIBOLD)
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
    container::Style {
        background: Some(iced::Background::Color(if is_dark(t) { c("#0E1116") } else { c("#FFFFFF") })),
        ..Default::default()
    }
}

