use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{column, container, row, svg, text};
use iced::Length::Fixed;
use iced::{Alignment, Background, Element, Fill, Padding, Theme};

use crate::app::{McScan, Message, ModalKind};
use crate::components::{action_button, ui::{btn, divider, BtnVariant}};
use crate::styles::{c, is_dark, MONO, SANS, SANS_SEMIBOLD};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let tr = app.tr();
    let settings_icon = svg::Handle::from_path(format!("{}/assets/settings.svg", env!("CARGO_MANIFEST_DIR")));
    let plus_icon = svg::Handle::from_path(format!("{}/assets/plus.svg", env!("CARGO_MANIFEST_DIR")));

    let top_row = row![
        action_button::render(app),
        Space::new().width(8),
        btn(BtnVariant::Icon { handle: settings_icon, size: 20.0 }, Message::OpenModal(ModalKind::Settings)),
    ]
    .align_y(Alignment::Center);

    let range_count = app.address_list.values().len();
    let total_hosts = app.address_list.total_hosts();
    let total_str = if total_hosts == u64::MAX { "∞".to_string() } else { total_hosts.to_string() };

    let ranges_header = row![
        text(format!("{} · {}", tr.ip_ranges, range_count)).size(11).font(SANS_SEMIBOLD)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
            }),
        Space::new().width(Fill),
        btn(BtnVariant::Icon { handle: plus_icon, size: 12.0 }, Message::OpenModal(ModalKind::AddRanges)),
    ]
    .align_y(Alignment::Center);

    let total_row = row![
        text(tr.total_addresses).size(12).font(SANS)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#5C636F") } else { c("#5a626c") }),
            }),
        Space::new().width(Fill),
        text(total_str).size(12).font(MONO)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#A2ABBA") } else { c("#3A4049") }),
            }),
    ]
    .align_y(Alignment::Center);

    container(
        column![
            top_row,
            Space::new().height(16),
            ranges_header,
            Space::new().height(6),
            app.address_list.view().map(Message::AddressList),
            Space::new().height(8),
            divider(),
            Space::new().height(12),
            total_row,
        ]
        .padding(Padding::from([16, 16]))
        .width(Fill)
        .height(Fill),
    )
    .style(panel_style)
    .width(Fixed(340.0))
    .height(Fill)
    .into()
}

fn panel_style(t: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Some(Background::Color(if is_dark(t) { c("#0B0E13") } else { c("#FFFFFF") })),
        ..Default::default()
    }
}
