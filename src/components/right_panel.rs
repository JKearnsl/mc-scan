use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{button, column, container, row, svg, text};
use iced::Length::Fixed;
use iced::{Alignment, Background, ContentFit, Element, Fill, Padding, Theme};

use crate::app::{McScan, Message, ModalKind};
use crate::components::ui::divider;
use crate::styles::{button_danger, button_primary, c, icon_button_style, is_dark, MONO, SANS, SANS_SEMIBOLD};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let top_row = row![action_button(app), Space::new().width(8), settings_button()]
        .align_y(Alignment::Center);

    let range_count = app.address_list.values().len();
    let total_hosts = app.address_list.total_hosts();
    let total_str = if total_hosts == u64::MAX { "∞".to_string() } else { total_hosts.to_string() };

    let ranges_header = row![
        text(format!("IP-ДИАПАЗОНЫ · {}", range_count)).size(11).font(SANS_SEMIBOLD)
            .style(|t: &Theme| iced::widget::text::Style {
                color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
            }),
        Space::new().width(Fill),
        button(
            container("+").center(Fill)
        )
            .on_press(Message::OpenModal(ModalKind::AddRanges))
            .padding(Padding::ZERO)
            .style(add_btn_style)
            .width(Fixed(26.0))
            .height(Fixed(26.0))
    ]
    .align_y(Alignment::Center);

    let total_row = row![
        text("Всего адресов").size(12).font(SANS)
            .style(|t: &Theme| iced::widget::text::Style {
                color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
            }),
        Space::new().width(Fill),
        text(total_str).size(12).font(MONO)
            .style(|t: &Theme| iced::widget::text::Style {
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
            Space::new().height(8),
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

fn settings_button() -> Element<'static, Message> {
    let icon = svg::Handle::from_path(format!(
        "{}/assets/settings.svg",
        env!("CARGO_MANIFEST_DIR")
    ));
    button(
        svg(icon)
            .content_fit(ContentFit::Fill)
            .width(Fixed(20.0))
            .height(Fixed(20.0))
            .style(|t: &Theme, _| svg::Style {
                color: Some(if is_dark(t) { c("#6B7480") } else { c("#5B6470") }),
            }),
    )
    .style(icon_button_style)
    .padding(Padding::from([13, 13]))
    .width(Fixed(48.0))
    .height(Fixed(48.0))
    .on_press(Message::OpenModal(ModalKind::Settings))
    .into()
}

fn action_button(app: &McScan) -> Element<'_, Message> {
    if app.is_scanning {
        return button(
            container(
                text("■  Стоп").size(16).font(SANS_SEMIBOLD)
                    .style(|_: &Theme| iced::widget::text::Style { color: Some(c("#FFFFFF")) }),
            )
            .center(Fill),
        )
        .style(button_danger)
        .on_press(Message::ScanStop)
        .width(Fill)
        .height(Fixed(48.0))
        .into();
    }

    let can_scan = !app.address_list.values().is_empty();
    let btn = button(
        container(
            text("▶  Сканировать").size(16).font(SANS_SEMIBOLD)
                .style(move |t: &Theme| iced::widget::text::Style {
                    color: Some(if can_scan {
                        if is_dark(t) { c("#08110B") } else { c("#FFFFFF") }
                    } else {
                        if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }
                    }),
                }),
        )
        .center(Fill),
    )
    .style(button_primary)
    .width(Fill)
    .height(Fixed(48.0));

    if can_scan { btn.on_press(Message::ScanStart).into() } else { btn.into() }
}

fn panel_style(t: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Some(Background::Color(if is_dark(t) { c("#0B0E13") } else { c("#FFFFFF") })),
        ..Default::default()
    }
}

fn add_btn_style(t: &Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
    use iced::widget::button::{Status, Style};
    use iced::{Border, Shadow};
    let dark = is_dark(t);
    let bg       = if dark { c("#181D25") } else { c("#F6F7F9") };
    let bg_hov   = if dark { c("#1F2630") } else { c("#EEF0F3") };
    let bg_prs   = if dark { c("#232A34") } else { c("#E5E8EC") };
    let text     = if dark { c("#6B7480") } else { c("#5B6470") };
    let text_hov = if dark { c("#E8EBF0") } else { c("#18A862") };
    let border_c = if dark { c("#232A34") } else { c("#E1E5EA") };
    let base = Style {
        background: Some(Background::Color(bg)), text_color: text,
        border: Border { color: border_c, width: 1.0, radius: 7.0.into() },
        shadow: Shadow::default(), snap: false,
    };
    match status {
        Status::Hovered => Style { background: Some(Background::Color(bg_hov)), text_color: text_hov, ..base },
        Status::Pressed => Style { background: Some(Background::Color(bg_prs)), ..base },
        _ => base,
    }
}
