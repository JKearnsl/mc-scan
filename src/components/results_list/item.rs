use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{column, container, row, text, text_input};
use iced::Length::Fixed;
use iced::{Alignment, Background, Border, Color, Element, Fill, Padding, Theme};

use crate::components::ui::chip;
use crate::i18n::Tr;
use crate::scanner::types::ServerInfo;
use crate::styles::{c, is_dark, MONO, MONO_SEMIBOLD, SANS, SANS_SEMIBOLD};

use super::avatar::build_avatar;
use super::ResultsListMessage;

pub fn server_card<'a>(info: &'a ServerInfo, tr: &'static Tr) -> Element<'a, ResultsListMessage> {
    let (name, description) = split_motd(&info.motd);
    let avatar = build_avatar(&name, &info.edition);
    let ip_port = format!("{}:{}", info.addr.ip(), info.addr.port());

    let mut left_col = column![
        text(name)
            .size(16)
            .font(SANS_SEMIBOLD)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#E8EBF0") } else { c("#161A20") }),
            })
            .wrapping(text::Wrapping::None),
    ];

    if !description.is_empty() {
        left_col = left_col.push(
            text(description)
                .size(13)
                .font(SANS)
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) { c("#A2ABBA") } else { c("#3A4049") }),
                })
                .wrapping(text::Wrapping::None),
        );
    }

    left_col = left_col.push(
        text_input("", &ip_port)
            .size(13)
            .font(MONO)
            .padding(Padding::ZERO)
            .style(|t: &Theme, _| {
                let color = if is_dark(t) { c("#6B7480") } else { c("#8A929E") };
                text_input::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 0.0.into() },
                    icon: Color::TRANSPARENT,
                    placeholder: color,
                    value: color,
                    selection: Color { r: 0.239, g: 0.839, b: 0.549, a: 0.25 },
                }
            }),
    );

    let left_block = left_col.spacing(3).width(Fill).clip(true);

    let ping_str = format!("{} ms", info.latency_ms);
    let (software, ver_str) = parse_version(&info.version);

    let right_block = row![
        players_column(info.online as u64, info.max_players as u64, tr.players),
        stat_column(tr.ping, ping_str, ping_color(info.latency_ms), Fixed(68.0)),
        version_column(ver_str, software, tr.version),
    ]
    .spacing(4)
    .align_y(Alignment::Start);

    container(
        row![avatar, Space::new().width(15), left_block, right_block]
            .align_y(Alignment::Center),
    )
    .style(card_style)
    .padding(Padding::from([13, 15]))
    .width(Fill)
    .into()
}

fn players_column(online: u64, max: u64, label: &'static str) -> Element<'static, ResultsListMessage> {
    column![
        text(label)
            .size(10)
            .font(SANS_SEMIBOLD)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
            }),
        row![
            text("●")
                .size(8)
                .font(MONO)
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) { c("#3DD68C") } else { c("#18A862") }),
                }),
            text(format!("{} / {}", online, max))
                .size(14)
                .font(MONO_SEMIBOLD)
                .style(|t: &Theme| iced::widget::text::Style {
                    color: Some(if is_dark(t) { c("#E8EBF0") } else { c("#161A20") }),
                }),
        ]
        .align_y(Alignment::Center)
        .spacing(6),
    ]
    .spacing(3)
    .align_x(iced::alignment::Horizontal::Right)
    .width(Fixed(84.0))
    .into()
}

fn stat_column(
    label: &'static str,
    value: String,
    value_color: Color,
    width: iced::Length,
) -> Element<'static, ResultsListMessage> {
    column![
        text(label)
            .size(10)
            .font(SANS_SEMIBOLD)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
            }),
        text(value)
            .size(14)
            .font(MONO_SEMIBOLD)
            .style(move |_: &Theme| iced::widget::text::Style { color: Some(value_color) }),
    ]
    .spacing(3)
    .align_x(iced::alignment::Horizontal::Right)
    .width(width)
    .into()
}

fn version_column(version: String, software: Option<String>, label: &'static str) -> Element<'static, ResultsListMessage> {
    let mut col = column![
        text(label)
            .size(10)
            .font(SANS_SEMIBOLD)
            .style(|t: &Theme| iced::widget::text::Style {
                color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
            }),
        text(version)
            .size(13)
            .font(MONO)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#A2ABBA") } else { c("#3A4049") }),
            }),
    ]
    .spacing(3)
    .align_x(iced::alignment::Horizontal::Right);

    if let Some(sw) = software {
        col = col.push(chip(sw));
    }

    col.width(Fixed(150.0)).into()
}

fn card_style(t: &Theme) -> ContainerStyle {
    let dark = is_dark(t);
    ContainerStyle {
        background: Some(Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") })),
        border: Border { color: if dark { c("#232A34") } else { c("#E5E9EF") }, width: 1.0, radius: 10.0.into() },
        ..Default::default()
    }
}

fn ping_color(ms: u64) -> Color {
    if ms < 80 { c("#3DD68C") } else if ms <= 200 { c("#E0B23C") } else { c("#E5604D") }
}

fn parse_version(raw: &str) -> (Option<String>, String) {
    if let Some(pos) = raw.find(' ') {
        let prefix = &raw[..pos];
        let rest = raw[pos + 1..].trim();
        if !prefix.is_empty() && prefix.chars().next().map_or(false, |c| c.is_ascii_alphabetic()) {
            return (Some(prefix.to_string()), rest.to_string());
        }
    }
    (None, raw.to_string())
}

fn split_motd(motd: &str) -> (String, String) {
    let stripped = motd.trim();
    if let Some((first, rest)) = stripped.split_once('\n') {
        (first.trim().to_string(), rest.trim().to_string())
    } else {
        (stripped.to_string(), String::new())
    }
}
