use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{button, column, container, mouse_area, row, svg, text};
use iced::Length::Fixed;
use iced::mouse::Interaction;
use iced::{
    Alignment, Background, Border, Color, ContentFit, Element, Fill, Padding, Shadow, Theme,
};
use crate::components::ui::{btn, scrollbar as styled_scroll, BtnVariant};

use crate::app::{McScan, Message, ModalKind};
use crate::scanner::types::Edition;
use crate::styles::{c, is_dark, MONO, MONO_SEMIBOLD, SANS, SANS_SEMIBOLD};

pub fn render(app: &McScan) -> Element<'_, Message> {
    let addr = match &app.modal {
        ModalKind::ServerPreview(a) => *a,
        _ => return Space::new().into(),
    };

    let server = match app.results.get_by_addr(addr) {
        Some(s) => s,
        None => return Space::new().into(),
    };

    let tr = app.tr();
    let close_icon = svg::Handle::from_path(format!("{}/assets/close.svg", env!("CARGO_MANIFEST_DIR")));
    let copy_icon  = svg::Handle::from_path(format!("{}/assets/copy.svg",  env!("CARGO_MANIFEST_DIR")));

    let (software, mc_version) = split_version(&server.version);
    let edition_str = match server.edition {
        Edition::Java    => tr.java_edition,
        Edition::Bedrock => tr.bedrock_edition,
    };
    let copy_label = if app.copied { tr.copied } else { tr.copy };
    let server_name = motd_first_line(&server.motd);

    let avatar = build_avatar_large(&server_name, &server.edition);

    let online_dot = container(Space::new())
        .width(Fixed(7.0))
        .height(Fixed(7.0))
        .style(|t: &Theme| ContainerStyle {
            background: Some(Background::Color(acc_green(t))),
            border: Border { radius: 99.0.into(), ..Default::default() },
            ..Default::default()
        });

    let online_row = row![
        online_dot,
        text(tr.online)
            .size(13)
            .font(SANS_SEMIBOLD)
            .style(|t: &Theme| text::Style { color: Some(acc_green(t)) }),
    ]
    .spacing(7)
    .align_y(Alignment::Center);

    let close_btn = btn(BtnVariant::Icon { handle: close_icon, size: 14.0 }, Message::CloseModal);

    let header = container(
        row![
            avatar,
            Space::new().width(16),
            column![
                text(server_name)
                    .size(17)
                    .font(SANS_SEMIBOLD)
                    .style(|t: &Theme| text::Style {
                        color: Some(if is_dark(t) { c("#E8EBF0") } else { c("#161A20") }),
                    })
                    .wrapping(text::Wrapping::None),
                Space::new().height(6),
                online_row,
            ]
            .width(Fill),
            close_btn,
        ]
        .align_y(Alignment::Start),
    )
    .padding(Padding::from([20, 22]))
    .width(Fill);

    let separator = container(Space::new())
        .width(Fill)
        .height(Fixed(1.0))
        .style(|t: &Theme| ContainerStyle {
            background: Some(Background::Color(sep_color(t))),
            ..Default::default()
        });

    let addr_str = format!("{}:{}", server.addr.ip(), server.addr.port());

    let addr_row = row![
        container(
            text(addr_str)
                .size(13)
                .font(MONO)
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) { c("#E8EBF0") } else { c("#161A20") }),
                }),
        )
        .style(inset_box_style)
        .padding(Padding::from([10, 13]))
        .width(Fill),
        Space::new().width(10),
        button(
            row![
                svg(copy_icon.clone())
                    .content_fit(ContentFit::Fill)
                    .width(Fixed(13.0))
                    .height(Fixed(13.0))
                    .style(|t: &Theme, _| svg::Style {
                        color: Some(acc_green(t)),
                    }),
                Space::new().width(7),
                text(copy_label)
                    .size(13)
                    .font(SANS_SEMIBOLD)
                    .style(|t: &Theme| text::Style { color: Some(acc_green(t)) }),
            ]
            .align_y(Alignment::Center),
        )
        .on_press(Message::CopyAddress)
        .style(copy_btn_style)
        .padding(Padding::from([10, 13]))
        .height(Fixed(36.5)),
    ]
    .align_y(Alignment::Center);

    let addr_section = labeled_section(tr.address, addr_row.into());

    let motd_text = server.motd.trim().to_string();
    let motd_section = labeled_section(
        tr.motd,
        container(
            text(if motd_text.is_empty() { "—".to_string() } else { motd_text })
                .size(13)
                .font(SANS)
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) { c("#A2ABBA") } else { c("#4A5260") }),
                }),
        )
        .style(inset_box_style)
        .padding(Padding::from([11, 13]))
        .width(Fill)
        .into(),
    );

    let stats_block = container(
        column![
            row![
                stat_cell(tr.players, format!("{} / {}", server.online, server.max_players), false),
                Space::new().width(9),
                stat_cell_colored(tr.ping, format!("{} ms", server.latency_ms), ping_color(server.latency_ms)),
                Space::new().width(9),
                stat_cell(tr.edition, edition_str.to_string(), false),
            ],
            Space::new().height(9),
            row![
                stat_cell(tr.version, mc_version, true),
                Space::new().width(9),
                stat_cell(tr.protocol, server.protocol.to_string(), true),
                Space::new().width(9),
                stat_cell(tr.software, software.unwrap_or_else(|| "—".to_string()), false),
            ],
        ]
        .width(Fill),
    )
    .padding(Padding::from([10, 22]));

    let chart_block = labeled_section(
        tr.latency,
        ping_chart(server.latency_ms, &server.ping_history),
    );

    let samples_block = if !server.samples.is_empty() {
        let names: Vec<&str> = server.samples.iter().take(20).map(|s| s.as_str()).collect();
        let mut chips_col = column![].spacing(7);
        for chunk in names.chunks(4) {
            let mut r = row![].spacing(7);
            for name in chunk {
                r = r.push(player_chip(name));
            }
            chips_col = chips_col.push(r);
        }
        Some(labeled_section(tr.players_online, chips_col.into()))
    } else {
        None
    };

    let bottom = container(
        row![
            button(container(
                text(copy_label).size(14).font(SANS_SEMIBOLD)
                    .style(|t: &Theme| text::Style {
                        color: Some(if is_dark(t) { c("#08110B") } else { c("#FFFFFF") }),
                    })
            ).center(Fill))
            .on_press(Message::CopyAddress)
            .style(primary_btn_style)
            .width(Fill)
            .height(Fixed(44.0)),
            Space::new().width(11),
            button(container(
                text(tr.done).size(14).font(SANS_SEMIBOLD)
            ).center(Fill))
            .on_press(Message::CloseModal)
            .style(secondary_btn_style)
            .width(Fill)
            .height(Fixed(44.0)),
        ]
        .width(Fill),
    )
    .padding(Padding { top: 8.0, right: 22.0, bottom: 20.0, left: 22.0 });

    let mut body = column![
        header,
        separator,
        addr_section,
        motd_section,
        stats_block,
        chart_block,
    ]
    .width(Fill);

    if let Some(s) = samples_block {
        body = body.push(s);
    }
    body = body.push(bottom);

    let dialog = container(
        styled_scroll(body),
    )
    .width(Fixed(516.0))
    .height(Fixed(535.0))
    .padding(Padding { top: 8.0, bottom: 8.0, ..Default::default() })
    .style(dialog_bg_style);

    mouse_area(
        container(mouse_area(dialog).on_press(Message::NoOp))
            .center_x(Fill)
            .center_y(Fill)
            .style(|_: &Theme| ContainerStyle {
                background: Some(Background::Color(Color { r: 0.02, g: 0.03, b: 0.04, a: 0.76 })),
                ..Default::default()
            }),
    )
    .on_press(Message::CloseModal)
    .interaction(Interaction::Idle)
    .into()
}


fn ping_chart<'a>(current_ping: u64, history: &[u64]) -> Element<'a, Message> {
    let total = 30usize;
    let hist_start = total.saturating_sub(history.len());
    let max_p = history.iter().copied().max()
        .unwrap_or(current_ping)
        .max(current_ping)
        .max(1) as f32;

    let bars: Vec<Element<'a, Message>> = (0..total)
        .map(|i| {
            let (h, opacity) = if i < hist_start {
                let sim = 7.0 + ((i as f64 * 0.9 + current_ping as f64 * 0.013).sin().abs() * 31.0) as f32;
                (sim, 0.30 + (i as f32 / total as f32) * 0.70)
            } else {
                let p = history[i - hist_start] as f32;
                let h = 7.0 + (p / max_p).min(1.0) * 31.0;
                let j = i - hist_start;
                (h, (0.30 + (j as f32 / history.len().max(1) as f32) * 0.70).min(1.0))
            };
            let color = ping_color(current_ping);
            let bar_color = Color { a: opacity, ..color };

            container(Space::new())
                .width(Fill)
                .height(Fixed(h))
                .style(move |_: &Theme| ContainerStyle {
                    background: Some(Background::Color(bar_color)),
                    border: Border { radius: 2.0.into(), ..Default::default() },
                    ..Default::default()
                })
                .into()
        })
        .collect();

    row(bars).spacing(2).height(Fixed(44.0)).align_y(Alignment::End).into()
}


fn player_chip(name: &str) -> Element<'_, Message> {
    let first = name.chars().find(|c| c.is_alphanumeric()).unwrap_or('A') as u32;
    let idx = ((first.wrapping_mul(2654435769)) >> 28) as usize;
    const DOT_COLORS: &[Color] = &[
        Color { r: 0.239, g: 0.839, b: 0.549, a: 1.0 },
        Color { r: 0.561, g: 0.702, b: 1.000, a: 1.0 },
        Color { r: 0.878, g: 0.698, b: 0.478, a: 1.0 },
        Color { r: 0.753, g: 0.478, b: 0.878, a: 1.0 },
        Color { r: 0.878, g: 0.478, b: 0.478, a: 1.0 },
        Color { r: 0.478, g: 0.831, b: 0.878, a: 1.0 },
        Color { r: 0.639, g: 0.851, b: 0.478, a: 1.0 },
        Color { r: 0.878, g: 0.753, b: 0.478, a: 1.0 },
    ];
    let dot_color = DOT_COLORS[idx % DOT_COLORS.len()];

    container(
        row![
            container(Space::new())
                .width(Fixed(7.0))
                .height(Fixed(7.0))
                .style(move |_: &Theme| ContainerStyle {
                    background: Some(Background::Color(dot_color)),
                    border: Border { radius: 3.0.into(), ..Default::default() },
                    ..Default::default()
                }),
            text(name.to_string())
                .size(13)
                .font(SANS)
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) { c("#A2ABBA") } else { c("#3A4049") }),
                }),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .style(|t: &Theme| ContainerStyle {
        background: Some(Background::Color(if is_dark(t) { c("#191E27") } else { c("#EEF0F3") })),
        border: Border {
            color: if is_dark(t) { c("#262E3C") } else { c("#DDE2E8") },
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .padding(Padding { top: 6.0, right: 10.0, bottom: 6.0, left: 8.0 })
    .into()
}


fn build_avatar_large<'a>(name: &str, edition: &Edition) -> Element<'a, Message> {
    use iced::gradient;
    use iced::widget::Stack;

    let first = name.chars().find(|c| c.is_alphanumeric()).unwrap_or('?')
        .to_uppercase().next().unwrap_or('?');

    let name_first = name.chars().find(|c| c.is_alphanumeric()).unwrap_or('A') as u32;
    let idx = ((name_first.wrapping_mul(2654435769)) >> 28) as usize;

    const PALETTES: &[(u32, u32, u32)] = &[
        (0x214a3a, 0x16302a, 0x3DD68C), (0x33405c, 0x222a3d, 0x8FB3FF),
        (0x4a3a28, 0x2f2418, 0xE0B27A), (0x3d2a4a, 0x2a1d35, 0xC07AE0),
        (0x4a2828, 0x301818, 0xE07A7A), (0x1a3a4a, 0x11252f, 0x7AD4E0),
        (0x2d3d1a, 0x1d2811, 0xA3D97A), (0x4a3828, 0x302415, 0xE0C07A),
        (0x28384a, 0x18242f, 0x7AB8E0), (0x3a2a3a, 0x251825, 0xE07AC0),
        (0x1a3a3a, 0x112525, 0x7AE0D4), (0x3a3a1a, 0x252511, 0xD4E07A),
        (0x3a1a1a, 0x251111, 0xE08C7A), (0x1a2a3a, 0x111a25, 0x7AAEE0),
        (0x2a3a2a, 0x182518, 0x9AE09A), (0x3a2a1a, 0x251a0f, 0xE0B07A),
    ];
    let (gs, ge, lc) = PALETTES[idx % PALETTES.len()];
    let grad_s = hex_to_color(gs);
    let grad_e = hex_to_color(ge);
    let letter_c = hex_to_color(lc);

    let letter_bg = container(
        text(first.to_string())
            .size(27)
            .font(crate::styles::SANS_SEMIBOLD)
            .style(move |_: &Theme| text::Style { color: Some(letter_c) }),
    )
    .style(move |_: &Theme| ContainerStyle {
        background: Some(Background::Gradient(
            gradient::Linear::new(std::f32::consts::PI * 0.75)
                .add_stop(0.0, grad_s)
                .add_stop(1.0, grad_e)
                .into(),
        )),
        border: Border { radius: 14.0.into(), ..Default::default() },
        ..Default::default()
    })
    .center(Fixed(68.0));

    let (badge_bg, badge_fg, badge_ch) = match edition {
        Edition::Java    => (c("#D99A3C"), c("#1A0E00"), "J"),
        Edition::Bedrock => (c("#13A884"), c("#001A14"), "B"),
    };

    let badge = container(
        container(
            text(badge_ch)
                .size(11)
                .font(crate::styles::MONO_SEMIBOLD)
                .style(move |_: &Theme| text::Style { color: Some(badge_fg) }),
        )
        .style(move |t: &Theme| ContainerStyle {
            background: Some(Background::Color(badge_bg)),
            border: Border {
                color: if is_dark(t) { c("#181C21") } else { c("#FFFFFF") },
                width: 2.5,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .center(Fixed(22.0)),
    )
    .width(Fixed(72.0))
    .height(Fixed(72.0))
    .align_right(Fixed(72.0))
    .align_bottom(Fixed(72.0));

    let bg_layer = container(letter_bg)
        .width(Fixed(68.0))
        .height(Fixed(68.0))
        .align_x(iced::alignment::Horizontal::Left)
        .align_y(iced::alignment::Vertical::Top);

    Stack::new()
        .push(bg_layer)
        .push(badge)
        .width(Fixed(72.0))
        .height(Fixed(72.0))
        .into()
}


fn labeled_section<'a>(label: &'a str, content: Element<'a, Message>) -> Element<'a, Message> {
    container(
        column![
            text(label)
                .size(10)
                .font(SANS_SEMIBOLD)
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
                }),
            Space::new().height(8),
            content,
        ]
        .width(Fill),
    )
    .padding(Padding::from([10, 22]))
    .into()
}

fn stat_cell<'a>(label: &'a str, value: String, mono: bool) -> Element<'a, Message> {
    container(
        column![
            text(label)
                .size(10)
                .font(SANS_SEMIBOLD)
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
                }),
            Space::new().height(5),
            text(value)
                .size(14)
                .font(if mono { MONO_SEMIBOLD } else { SANS_SEMIBOLD })
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) { c("#E8EBF0") } else { c("#161A20") }),
                }),
        ]
        .width(Fill),
    )
    .style(stat_card_style)
    .padding(Padding::from([10, 12]))
    .width(Fill)
    .into()
}

fn stat_cell_colored<'a>(label: &'a str, value: String, value_color: Color) -> Element<'a, Message> {
    container(
        column![
            text(label)
                .size(10)
                .font(SANS_SEMIBOLD)
                .style(|t: &Theme| text::Style {
                    color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
                }),
            Space::new().height(5),
            text(value)
                .size(14)
                .font(MONO_SEMIBOLD)
                .style(move |_: &Theme| text::Style { color: Some(value_color) }),
        ]
        .width(Fill),
    )
    .style(stat_card_style)
    .padding(Padding::from([10, 12]))
    .width(Fill)
    .into()
}

fn motd_first_line(motd: &str) -> String {
    motd.trim()
        .split('\n')
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}

fn split_version(raw: &str) -> (Option<String>, String) {
    if let Some(pos) = raw.find(' ') {
        let prefix = &raw[..pos];
        let rest = raw[pos + 1..].trim();
        if !prefix.is_empty() && prefix.chars().next().map_or(false, |c| c.is_ascii_alphabetic()) {
            return (Some(prefix.to_string()), rest.to_string());
        }
    }
    (None, raw.to_string())
}

fn ping_color(ms: u64) -> Color {
    if ms < 80 { c("#3DD68C") } else if ms <= 200 { c("#E0B23C") } else { c("#E5604D") }
}

fn hex_to_color(hex: u32) -> Color {
    Color {
        r: ((hex >> 16) & 0xFF) as f32 / 255.0,
        g: ((hex >> 8) & 0xFF) as f32 / 255.0,
        b: (hex & 0xFF) as f32 / 255.0,
        a: 1.0,
    }
}

fn acc_green(t: &Theme) -> Color {
    if is_dark(t) { c("#3DD68C") } else { c("#18A862") }
}

fn sep_color(t: &Theme) -> Color {
    if is_dark(t) { c("#1E2530") } else { c("#E5E9EF") }
}


fn dialog_bg_style(t: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Some(Background::Color(if is_dark(t) { c("#181C21") } else { c("#FFFFFF") })),
        border: Border {
            color: if is_dark(t) { c("#2A3240") } else { c("#D8DDE4") },
            width: 1.0,
            radius: 16.0.into(),
        },
        ..Default::default()
    }
}

fn inset_box_style(t: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Some(Background::Color(if is_dark(t) { c("#0E1115") } else { c("#F6F8FA") })),
        border: Border {
            color: sep_color(t),
            width: 1.0,
            radius: 9.0.into(),
        },
        ..Default::default()
    }
}

fn stat_card_style(t: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Some(Background::Color(if is_dark(t) { c("#131821") } else { c("#F6F8FA") })),
        border: Border {
            color: sep_color(t),
            width: 1.0,
            radius: 9.0.into(),
        },
        ..Default::default()
    }
}


fn copy_btn_style(t: &Theme, status: button::Status) -> button::Style {
    let dark = is_dark(t);
    let (r, g, b) = if dark { (0.239f32, 0.839, 0.549) } else { (0.094, 0.659, 0.384) };
    let bg_a = match status {
        button::Status::Hovered => 0.22f32,
        button::Status::Pressed => 0.30,
        _ => 0.14,
    };
    button::Style {
        background: Some(Background::Color(Color { r, g, b, a: bg_a })),
        text_color: if dark { c("#3DD68C") } else { c("#18A862") },
        border: Border { color: Color { r, g, b, a: 0.35 }, width: 1.0, radius: 9.0.into() },
        shadow: Shadow::default(),
        snap: false,
    }
}

fn primary_btn_style(t: &Theme, status: button::Status) -> button::Style {
    let dark = is_dark(t);
    let accent     = if dark { c("#3DD68C") } else { c("#18A862") };
    let accent_hov = if dark { c("#34C27E") } else { c("#138A52") };
    let accent_prs = if dark { c("#2BAD6F") } else { c("#0F7040") };
    let bg = match status {
        button::Status::Hovered => accent_hov,
        button::Status::Pressed => accent_prs,
        _ => accent,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: if dark { c("#08110B") } else { c("#FFFFFF") },
        border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 10.0.into() },
        shadow: Shadow::default(),
        snap: false,
    }
}

fn secondary_btn_style(t: &Theme, status: button::Status) -> button::Style {
    let dark = is_dark(t);
    let bg = match status {
        button::Status::Hovered => if dark { c("#1E2530") } else { c("#F2F4F7") },
        button::Status::Pressed => if dark { c("#262E3C") } else { c("#E8ECF0") },
        _ => if dark { c("#131821") } else { c("#FFFFFF") },
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: if dark { c("#E8EBF0") } else { c("#161A20") },
        border: Border {
            color: if dark { c("#2A3240") } else { c("#DDE2E8") },
            width: 1.0,
            radius: 10.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}
