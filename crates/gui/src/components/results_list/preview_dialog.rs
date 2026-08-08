use crate::components::ui::{
    BtnVariant, body, btn, caption, cell, cell_colored, chip, chip_dot, field, heading,
    scrollbar as styled_scroll, wrap,
};
use crate::text::strip_section_codes;
use iced::Length::Fixed;
use iced::mouse::Interaction;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{button, column, container, mouse_area, row, svg, text};
use iced::{
    Alignment, Background, Border, Color, ContentFit, Element, Fill, Padding, Shadow, Theme,
};

use super::avatar::{AvatarSize, build_avatar_icon};
use crate::app::{McScan, Message, ModalKind};
use crate::styles::{MONO, MONO_SEMIBOLD, SANS_SEMIBOLD, c, is_dark};
use scanner::types::Edition;

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
    let close_icon = crate::components::ui::icons::close();
    let copy_icon = crate::components::ui::icons::copy();

    let (software, mc_version) = super::parse_version(&strip_section_codes(&server.version));
    let edition_str = match server.edition {
        Edition::Java => tr.java_edition,
        Edition::Bedrock => tr.bedrock_edition,
    };
    let copy_label = if app.copied { tr.copied } else { tr.copy };
    let server_name = motd_first_line(&server.motd);

    let avatar = build_avatar_icon(
        &server_name,
        &server.edition,
        app.results.avatar_large(addr),
        AvatarSize::LARGE,
        |t: &Theme| {
            if is_dark(t) {
                c("#181C21")
            } else {
                c("#FFFFFF")
            }
        },
    );

    let online_dot = container(Space::new())
        .width(Fixed(7.0))
        .height(Fixed(7.0))
        .style(|t: &Theme| ContainerStyle {
            background: Some(Background::Color(acc_green(t))),
            border: Border {
                radius: 99.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

    let online_row = row![
        online_dot,
        text(tr.online)
            .size(13)
            .font(SANS_SEMIBOLD)
            .style(|t: &Theme| text::Style {
                color: Some(acc_green(t))
            }),
    ]
    .spacing(7)
    .align_y(Alignment::Center);

    let close_btn = btn(
        BtnVariant::Icon {
            handle: close_icon,
            size: 14.0,
        },
        Message::CloseModal,
    );

    let header = container(
        row![
            avatar,
            Space::new().width(16),
            column![heading(server_name), Space::new().height(6), online_row,]
                .width(Fill)
                .clip(true),
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
                    color: Some(if is_dark(t) {
                        c("#E8EBF0")
                    } else {
                        c("#161A20")
                    }),
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
                    .style(|t: &Theme| text::Style {
                        color: Some(acc_green(t))
                    }),
            ]
            .align_y(Alignment::Center),
        )
        .on_press(Message::CopyAddress)
        .style(copy_btn_style)
        .padding(Padding::from([10, 13]))
        .height(Fixed(36.5)),
    ]
    .align_y(Alignment::Center);

    let addr_section = field(tr.address, addr_row.into());

    let motd_text = strip_section_codes(server.motd.trim());
    let motd_section = field(
        tr.motd,
        container(body(if motd_text.is_empty() {
            "—".to_string()
        } else {
            motd_text
        }))
        .style(inset_box_style)
        .padding(Padding::from([11, 13]))
        .width(Fill)
        .into(),
    );

    let mut extra_cells: Vec<Element<'_, Message>> = Vec::new();
    if let Some(w) = &server.world {
        extra_cells.push(cell(tr.world, strip_section_codes(w), false));
    }
    if let Some(gm) = &server.gamemode {
        extra_cells.push(cell(tr.gamemode, strip_section_codes(gm), false));
    }
    if let Some(sc) = server.secure_chat {
        let v = if sc { tr.enabled } else { tr.disabled };
        extra_cells.push(cell(tr.secure_chat, v.to_string(), false));
    }
    if let Some(om) = server.online_mode {
        let v = if om { tr.online_yes } else { tr.online_no };
        extra_cells.push(cell(tr.online_mode, v.to_string(), false));
    }
    if !server.mods.is_empty() {
        extra_cells.push(cell(tr.mods, server.mods.len().to_string(), true));
    }

    let version_expandable = is_version_expandable(&mc_version);
    let version_cell = if version_expandable {
        expandable_version_cell(tr.version, &mc_version, app.version_expanded)
    } else {
        cell(tr.version, mc_version.clone(), true)
    };

    let mut stats_col = column![
        row![
            cell(
                tr.players,
                format!("{} / {}", server.online, server.max_players),
                false
            ),
            Space::new().width(9),
            cell_colored(
                tr.ping,
                format!("{} ms", server.latency_ms),
                ping_color(server.latency_ms)
            ),
            Space::new().width(9),
            cell(tr.edition, edition_str.to_string(), false),
        ],
        Space::new().height(9),
        row![
            version_cell,
            Space::new().width(9),
            cell(tr.protocol, server.protocol.to_string(), true),
            Space::new().width(9),
            cell(
                tr.software,
                software.unwrap_or_else(|| "—".to_string()),
                false
            ),
        ],
    ]
    .width(Fill);

    if !extra_cells.is_empty() {
        let mut extra_row = row![].width(Fill);
        for (i, el) in extra_cells.into_iter().enumerate() {
            if i > 0 {
                extra_row = extra_row.push(Space::new().width(9));
            }
            extra_row = extra_row.push(el);
        }
        stats_col = stats_col.push(Space::new().height(9)).push(extra_row);
    }

    let stats_block = container(stats_col).padding(Padding::from([10, 22]));

    let chart_block = field(
        tr.latency,
        ping_chart(server.latency_ms, &server.ping_history),
    );

    let samples_block = if !server.samples.is_empty() {
        let chips: Vec<Element<'_, Message>> = server
            .samples
            .iter()
            .take(20)
            .map(|s| {
                let name = strip_section_codes(s);
                let color = player_dot_color(&name);
                chip_dot(name, color)
            })
            .collect();
        Some(field(tr.players_online, wrap(chips).spacing(7.0).into()))
    } else {
        None
    };

    let mods_block = if !server.mods.is_empty() {
        let chips: Vec<Element<'_, Message>> = server
            .mods
            .iter()
            .map(|m| {
                let label = if m.version.is_empty() {
                    m.id.clone()
                } else {
                    format!("{} {}", m.id, m.version)
                };
                chip(strip_section_codes(&label))
            })
            .collect();
        let title = format!("{} · {}", tr.mods, server.mods.len());
        Some(field(title, wrap(chips).spacing(7.0).into()))
    } else {
        None
    };

    let plugins_block = if !server.plugins.is_empty() {
        let chips: Vec<Element<'_, Message>> = server
            .plugins
            .iter()
            .map(|p| chip(strip_section_codes(p)))
            .collect();
        let title = format!("{} · {}", tr.plugins, server.plugins.len());
        Some(field(title, wrap(chips).spacing(7.0).into()))
    } else {
        None
    };

    let mut col = column![header, separator, addr_section, motd_section, stats_block,].width(Fill);

    col = col.push(chart_block);

    if let Some(s) = mods_block {
        col = col.push(s);
    }

    if let Some(s) = plugins_block {
        col = col.push(s);
    }

    if let Some(s) = samples_block {
        col = col.push(s);
    }

    let dialog = container(styled_scroll(col))
        .width(Fixed(516.0))
        .height(Fixed(535.0))
        .padding(Padding {
            top: 8.0,
            bottom: 8.0,
            ..Default::default()
        })
        .style(dialog_bg_style);

    mouse_area(
        container(mouse_area(dialog).on_press(Message::NoOp))
            .center_x(Fill)
            .center_y(Fill)
            .style(|_: &Theme| ContainerStyle {
                background: Some(Background::Color(Color {
                    r: 0.02,
                    g: 0.03,
                    b: 0.04,
                    a: 0.76,
                })),
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
    let max_p = history
        .iter()
        .copied()
        .max()
        .unwrap_or(current_ping)
        .max(current_ping)
        .max(1) as f32;

    let bars: Vec<Element<'a, Message>> = (0..total)
        .map(|i| {
            let (h, opacity) = if i < hist_start {
                let sim = 7.0
                    + ((i as f64 * 0.9 + current_ping as f64 * 0.013).sin().abs() * 31.0) as f32;
                (sim, 0.30 + (i as f32 / total as f32) * 0.70)
            } else {
                let p = history[i - hist_start] as f32;
                let h = 7.0 + (p / max_p).min(1.0) * 31.0;
                let j = i - hist_start;
                (
                    h,
                    (0.30 + (j as f32 / history.len().max(1) as f32) * 0.70).min(1.0),
                )
            };
            let color = ping_color(current_ping);
            let bar_color = Color {
                a: opacity,
                ..color
            };

            container(Space::new())
                .width(Fill)
                .height(Fixed(h))
                .style(move |_: &Theme| ContainerStyle {
                    background: Some(Background::Color(bar_color)),
                    border: Border {
                        radius: 2.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into()
        })
        .collect();

    row(bars)
        .spacing(2)
        .height(Fixed(44.0))
        .align_y(Alignment::End)
        .into()
}

fn is_version_expandable(v: &str) -> bool {
    let v = v.trim();
    !v.is_empty() && (v.contains(',') || v.chars().count() > 14)
}

fn version_summary(v: &str) -> String {
    let tokens: Vec<&str> = v
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if tokens.len() >= 2 {
        return format!("{} +{}", tokens[0], tokens.len() - 1);
    }
    let v = v.trim();
    if v.chars().count() > 14 {
        let head: String = v.chars().take(13).collect();
        format!("{head}…")
    } else {
        v.to_string()
    }
}

fn expandable_version_cell<'a>(
    label: &'a str,
    value: &str,
    expanded: bool,
) -> Element<'a, Message> {
    let chevron = if expanded {
        crate::components::ui::icons::chevron_up()
    } else {
        crate::components::ui::icons::chevron_down()
    };

    let header = row![
        container(caption(label, 10)).width(Fill),
        Space::new().width(6),
        svg(chevron)
            .content_fit(ContentFit::Contain)
            .width(Fixed(12.0))
            .height(Fixed(12.0))
            .style(|t: &Theme, _| svg::Style {
                color: Some(if is_dark(t) {
                    c("#6B7480")
                } else {
                    c("#8A929E")
                }),
            }),
    ]
    .align_y(Alignment::Center);

    let content: Element<'a, Message> = if expanded {
        let chips: Vec<Element<'a, Message>> = value
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| chip(s.to_string()))
            .collect();
        wrap(chips).spacing(6.0).into()
    } else {
        text(version_summary(value))
            .size(14)
            .width(Fill)
            .wrapping(text::Wrapping::None)
            .font(MONO_SEMIBOLD)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) {
                    c("#E8EBF0")
                } else {
                    c("#161A20")
                }),
            })
            .into()
    };

    button(
        column![header, Space::new().height(6), content]
            .width(Fill)
            .clip(true),
    )
    .on_press(Message::ToggleVersionExpand)
    .style(version_cell_style)
    .padding(Padding::from([10, 12]))
    .width(Fill)
    .into()
}

fn version_cell_style(t: &Theme, status: button::Status) -> button::Style {
    let dark = is_dark(t);
    let base = if dark { c("#131821") } else { c("#F6F8FA") };
    let hovered = if dark { c("#182030") } else { c("#EEF2F7") };
    let bg = match status {
        button::Status::Hovered | button::Status::Pressed => hovered,
        _ => base,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: if dark { c("#E8EBF0") } else { c("#161A20") },
        border: Border {
            color: sep_color(t),
            width: 1.0,
            radius: 9.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

fn motd_first_line(motd: &str) -> String {
    strip_section_codes(motd.trim())
        .split('\n')
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}

// Deterministic accent for a player nick so a given name keeps its dot color.
fn player_dot_color(name: &str) -> Color {
    const DOT_COLORS: &[Color] = &[
        Color {
            r: 0.239,
            g: 0.839,
            b: 0.549,
            a: 1.0,
        },
        Color {
            r: 0.561,
            g: 0.702,
            b: 1.000,
            a: 1.0,
        },
        Color {
            r: 0.878,
            g: 0.698,
            b: 0.478,
            a: 1.0,
        },
        Color {
            r: 0.753,
            g: 0.478,
            b: 0.878,
            a: 1.0,
        },
        Color {
            r: 0.878,
            g: 0.478,
            b: 0.478,
            a: 1.0,
        },
        Color {
            r: 0.478,
            g: 0.831,
            b: 0.878,
            a: 1.0,
        },
        Color {
            r: 0.639,
            g: 0.851,
            b: 0.478,
            a: 1.0,
        },
        Color {
            r: 0.878,
            g: 0.753,
            b: 0.478,
            a: 1.0,
        },
    ];
    let first = name.chars().find(|c| c.is_alphanumeric()).unwrap_or('A') as u32;
    let idx = ((first.wrapping_mul(2654435769)) >> 28) as usize;
    DOT_COLORS[idx % DOT_COLORS.len()]
}

fn ping_color(ms: u64) -> Color {
    if ms < 80 {
        c("#3DD68C")
    } else if ms <= 200 {
        c("#E0B23C")
    } else {
        c("#E5604D")
    }
}

fn acc_green(t: &Theme) -> Color {
    if is_dark(t) {
        c("#3DD68C")
    } else {
        c("#18A862")
    }
}

fn sep_color(t: &Theme) -> Color {
    if is_dark(t) {
        c("#1E2530")
    } else {
        c("#E5E9EF")
    }
}

fn dialog_bg_style(t: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Some(Background::Color(if is_dark(t) {
            c("#181C21")
        } else {
            c("#FFFFFF")
        })),
        border: Border {
            color: if is_dark(t) {
                c("#2A3240")
            } else {
                c("#D8DDE4")
            },
            width: 1.0,
            radius: 16.0.into(),
        },
        ..Default::default()
    }
}

fn inset_box_style(t: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Some(Background::Color(if is_dark(t) {
            c("#0E1115")
        } else {
            c("#F6F8FA")
        })),
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
    let (r, g, b) = if dark {
        (0.239f32, 0.839, 0.549)
    } else {
        (0.094, 0.659, 0.384)
    };
    let bg_a = match status {
        button::Status::Hovered => 0.22f32,
        button::Status::Pressed => 0.30,
        _ => 0.14,
    };
    button::Style {
        background: Some(Background::Color(Color { r, g, b, a: bg_a })),
        text_color: if dark { c("#3DD68C") } else { c("#18A862") },
        border: Border {
            color: Color { r, g, b, a: 0.35 },
            width: 1.0,
            radius: 9.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}
