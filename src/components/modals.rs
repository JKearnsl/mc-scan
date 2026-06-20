use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{button, column, container, row, text, text_editor, text_input};
use iced::Length::Fixed;
use iced::{Alignment, Background, Border, Color, Element, Fill, Padding, Theme};

use crate::app::{McScan, Message};
use crate::styles::{
    button_danger, button_primary, button_secondary, c, icon_button_style, is_dark,
    MONO, SANS, SANS_SEMIBOLD,
};

pub fn settings(app: &McScan) -> Element<'_, Message> {
    let is_dark_theme = app.is_dark;

    let dialog = container(
        column![
            dialog_title("Настройки"),
            Space::new().height(16),
            section_label("ТЕМА"),
            Space::new().height(8),
            row![
                button(container(text("Тёмная").size(13).font(SANS_SEMIBOLD)).center(Fill))
                    .style(if is_dark_theme { button_primary } else { button_secondary })
                    .on_press(Message::SetTheme(true))
                    .width(Fill)
                    .height(Fixed(36.0)),
                Space::new().width(8),
                button(container(text("Светлая").size(13).font(SANS_SEMIBOLD)).center(Fill))
                    .style(if !is_dark_theme { button_primary } else { button_secondary })
                    .on_press(Message::SetTheme(false))
                    .width(Fill)
                    .height(Fixed(36.0)),
            ],
            Space::new().height(16),
            section_label("ПОРТЫ"),
            Space::new().height(8),
            labeled_input("Java",       &app.settings.java_ports,    "25565", Message::JavaPortsChanged,    app.settings.java_ports_error),
            Space::new().height(6),
            labeled_input("Bedrock",    &app.settings.bedrock_ports,  "19132", Message::BedrockPortsChanged, app.settings.bedrock_ports_error),
            Space::new().height(16),
            section_label("ПАРАМЕТРЫ"),
            Space::new().height(8),
            labeled_input("Потоки",     &app.settings.concurrency,    "1024",  Message::ConcurrencyChanged,  false),
            Space::new().height(6),
            labeled_input("Таймаут мс", &app.settings.timeout_ms,     "1500",  Message::TimeoutChanged,      false),
            Space::new().height(20),
            button(
                container(
                    text("Готово").size(15).font(SANS_SEMIBOLD)
                        .style(|t: &Theme| iced::widget::text::Style {
                            color: Some(if is_dark(t) { c("#08110B") } else { c("#FFFFFF") }),
                        }),
                )
                .center(Fill),
            )
            .on_press(Message::CloseModal)
            .style(button_primary)
            .width(Fill)
            .height(Fixed(44.0)),
        ]
        .padding(Padding::from([20, 24])),
    )
    .width(Fixed(380.0))
    .style(dialog_style);

    backdrop(dialog.into())
}

pub fn add_ranges(app: &McScan) -> Element<'_, Message> {
    let dialog = container(
        column![
            dialog_title("Добавить диапазоны"),
            Space::new().height(4),
            text("CIDR (10.0.0.0/8) · диапазон (1.2.3.4-1.2.3.100) · одиночный IP")
                .size(11).font(SANS)
                .style(|t: &Theme| iced::widget::text::Style {
                    color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
                }),
            Space::new().height(12),
            text_editor(&app.ranges_editor)
                .on_action(Message::RangesEditorAction)
                .height(Fixed(160.0))
                .style(text_editor_style_fn)
                .font(MONO)
                .size(13)
                .padding(Padding::from([8, 10])),
            Space::new().height(16),
            row![
                button(
                    container(
                        text("Отмена").size(14).font(SANS_SEMIBOLD)
                            .style(|_: &Theme| iced::widget::text::Style { color: Some(c("#FFFFFF")) }),
                    )
                    .center(Fill),
                )
                .on_press(Message::CloseModal)
                .style(button_danger)
                .width(Fill)
                .height(Fixed(44.0)),
                Space::new().width(10),
                button(
                    container(
                        text("Добавить").size(14).font(SANS_SEMIBOLD)
                            .style(|t: &Theme| iced::widget::text::Style {
                                color: Some(if is_dark(t) { c("#08110B") } else { c("#FFFFFF") }),
                            }),
                    )
                    .center(Fill),
                )
                .on_press(Message::ConfirmAddRanges)
                .style(button_primary)
                .width(Fill)
                .height(Fixed(44.0)),
            ],
        ]
        .padding(Padding::from([20, 24])),
    )
    .width(Fixed(460.0))
    .style(dialog_style);

    backdrop(dialog.into())
}

fn dialog_title(title: &str) -> Element<'_, Message> {
    row![
        text(title).size(16).font(SANS_SEMIBOLD)
            .style(|t: &Theme| iced::widget::text::Style {
                color: Some(if is_dark(t) { c("#E8EBF0") } else { c("#161A20") }),
            }),
        Space::new().width(Fill),
        button("×")
            .on_press(Message::CloseModal)
            .style(icon_button_style)
            .padding(Padding::from([4, 10])),
    ]
    .align_y(Alignment::Center)
    .into()
}

fn section_label(label: &str) -> Element<'_, Message> {
    text(label)
        .size(11)
        .font(SANS_SEMIBOLD)
        .style(|t: &Theme| iced::widget::text::Style {
            color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
        })
        .into()
}

fn labeled_input<'a>(
    label: &'a str,
    value: &'a str,
    placeholder: &'a str,
    on_change: fn(String) -> Message,
    error: bool,
) -> Element<'a, Message> {
    let style: fn(&Theme, _) -> _ = if error { text_input_error_style_fn } else { text_input_style_fn };
    row![
        text(label)
            .size(13)
            .font(SANS)
            .width(Fixed(90.0))
            .style(|t: &Theme| iced::widget::text::Style {
                color: Some(if is_dark(t) { c("#A2ABBA") } else { c("#3A4049") }),
            }),
        text_input(placeholder, value)
            .on_input(on_change)
            .padding(Padding::from([7, 10]))
            .size(13)
            .font(MONO)
            .style(style)
            .width(Fill),
    ]
    .align_y(Alignment::Center)
    .spacing(8)
    .into()
}

fn dialog_style(t: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Some(Background::Color(if is_dark(t) { c("#0E1116") } else { c("#FFFFFF") })),
        border: Border {
            color: if is_dark(t) { c("#232A34") } else { c("#E1E5EA") },
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}

fn text_input_style_fn(t: &Theme, status: iced::widget::text_input::Status) -> iced::widget::text_input::Style {
    let dark = is_dark(t);
    let focused = matches!(status, iced::widget::text_input::Status::Focused { .. });
    let accent = if dark { c("#3DD68C") } else { c("#18A862") };
    let border_def = if dark { c("#232A34") } else { c("#DDE2E8") };
    iced::widget::text_input::Style {
        background: Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") }),
        border: Border { radius: 7.0.into(), width: 1.0, color: if focused { accent } else { border_def } },
        icon: Color::TRANSPARENT,
        placeholder: if dark { c("#5C636F") } else { c("#A0A7B1") },
        value: if dark { c("#E8EBF0") } else { c("#161A20") },
        selection: Color { r: 0.239, g: 0.839, b: 0.549, a: 0.25 },
    }
}

fn text_input_error_style_fn(t: &Theme, status: iced::widget::text_input::Status) -> iced::widget::text_input::Style {
    let dark = is_dark(t);
    let focused = matches!(status, iced::widget::text_input::Status::Focused { .. });
    let danger = if dark { c("#E5604D") } else { c("#CC3A28") };
    let danger_dim = if dark { c("#6B2020") } else { c("#EDA8A0") };
    iced::widget::text_input::Style {
        background: Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") }),
        border: Border { radius: 7.0.into(), width: 1.5, color: if focused { danger } else { danger_dim } },
        icon: Color::TRANSPARENT,
        placeholder: if dark { c("#5C636F") } else { c("#A0A7B1") },
        value: danger,
        selection: Color { r: 0.898, g: 0.376, b: 0.302, a: 0.25 },
    }
}

fn text_editor_style_fn(t: &Theme, _: iced::widget::text_editor::Status) -> iced::widget::text_editor::Style {
    let dark = is_dark(t);
    iced::widget::text_editor::Style {
        background: Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") }),
        border: Border { color: if dark { c("#232A34") } else { c("#DDE2E8") }, width: 1.0, radius: 7.0.into() },
        placeholder: if dark { c("#5C636F") } else { c("#A0A7B1") },
        value: if dark { c("#E8EBF0") } else { c("#161A20") },
        selection: Color { r: 0.239, g: 0.839, b: 0.549, a: 0.25 },
    }
}

fn backdrop(content: Element<'_, Message>) -> Element<'_, Message> {
    container(content)
        .center_x(Fill)
        .center_y(Fill)
        .style(|_: &Theme| ContainerStyle {
            background: Some(Background::Color(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.70 })),
            ..Default::default()
        })
        .into()
}
