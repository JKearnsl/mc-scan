use iced::widget::{button, container, text};
use iced::Length::Fixed;
use iced::{Element, Fill, Theme};

use crate::app::{McScan, Message};
use crate::styles::{button_danger, button_primary, c, is_dark, SANS_SEMIBOLD};

pub fn render(app: &McScan) -> Element<'_, Message> {
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
