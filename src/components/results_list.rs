use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{column, container, row, scrollable, text, Stack};
use iced::{gradient, Alignment, Background, Border, Color, Element, Fill, Padding};
use iced::Length::Fixed;
use crate::scanner::types::{Edition, ServerInfo};
use crate::styles::{c, ping_color, scrollable_style, surface_card_style, MONO, MONO_SEMIBOLD, SANS, SANS_SEMIBOLD};

#[derive(Default)]
pub struct ResultsList {
    items: Vec<ServerInfo>,
}

#[derive(Debug, Clone)]
pub enum ResultsListMessage {}

impl ResultsList {
    pub fn push(&mut self, info: ServerInfo) { self.items.push(info); }
    pub fn clear(&mut self) { self.items.clear(); }
    pub fn count(&self) -> usize { self.items.len() }

    pub fn view(&self, dark: bool) -> Element<'_, ResultsListMessage> {
        let label_c = if dark { c("#5C636F") } else { c("#A0A7B1") };
        if self.items.is_empty() {
            return container(
                text("Результаты появятся здесь после сканирования")
                    .size(14)
                    .font(SANS)
                    .style(move |_| iced::widget::text::Style { color: Some(label_c) }),
            )
            .center_x(Fill)
            .center_y(Fill)
            .into();
        }

        let mut col = column![].spacing(10).padding(Padding::from([12, 16]));
        for info in &self.items {
            col = col.push(server_row(info, dark));
        }

        scrollable(col)
            .style(scrollable_style)
            .width(Fill)
            .height(Fill)
            .into()
    }
}

fn server_row(info: &ServerInfo, dark: bool) -> Element<'_, ResultsListMessage> {
    let (name, description) = split_motd(&info.motd);
    let avatar = build_avatar(&name, &info.edition);
    let ip_port = format!("{}:{}", info.addr.ip(), info.addr.port());

    let text1 = if dark { c("#E8EBF0") } else { c("#161A20") };
    let text2 = if dark { c("#A2ABBA") } else { c("#3A4049") };
    let text3 = if dark { c("#6B7480") } else { c("#8A929E") };

    let mut left_col = column![
        text(name)
            .size(16)
            .font(SANS_SEMIBOLD)
            .style(move |_| iced::widget::text::Style { color: Some(text1) })
            .wrapping(text::Wrapping::None),
    ];

    if !description.is_empty() {
        left_col = left_col.push(
            text(description)
                .size(13)
                .font(SANS)
                .style(move |_| iced::widget::text::Style { color: Some(text2) })
                .wrapping(text::Wrapping::None),
        );
    }

    left_col = left_col.push(
        text(ip_port)
            .size(13)
            .font(MONO)
            .style(move |_| iced::widget::text::Style { color: Some(text3) }),
    );

    let left_block = left_col.spacing(3).width(Fill).clip(true);

    let ping_col = ping_color(info.latency_ms);
    let ping_str = format!("{} ms", info.latency_ms);
    let (software, ver_str) = parse_version(&info.version);
    let right_block = row![
        players_column(info.online as u64, info.max_players as u64, dark),
        stat_column("ПИНГ", ping_str, ping_col, Fixed(68.0), dark),
        version_column(ver_str, software, dark),
    ]
    .spacing(4)
    .align_y(Alignment::Start);

    container(
        row![avatar, Space::new().width(15), left_block, right_block]
            .align_y(Alignment::Center),
    )
    .style(surface_card_style)
    .padding(Padding::from([13, 15]))
    .width(Fill)
    .into()
}

fn players_column(online: u64, max: u64, dark: bool) -> Element<'static, ResultsListMessage> {
    let label_c = if dark { c("#5C636F") } else { c("#A0A7B1") };
    let dot_c   = if dark { c("#3DD68C") } else { c("#18A862") };
    let count_c = if dark { c("#E8EBF0") } else { c("#161A20") };

    column![
        text("ИГРОКИ")
            .size(10)
            .font(SANS_SEMIBOLD)
            .style(move |_| iced::widget::text::Style { color: Some(label_c) }),
        row![
            text("●").size(8).font(MONO)
                .style(move |_| iced::widget::text::Style { color: Some(dot_c) }),
            text(format!("{} / {}", online, max))
                .size(14)
                .font(MONO_SEMIBOLD)
                .style(move |_| iced::widget::text::Style { color: Some(count_c) }),
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
    dark: bool,
) -> Element<'static, ResultsListMessage> {
    let label_c = if dark { c("#5C636F") } else { c("#A0A7B1") };
    column![
        text(label)
            .size(10)
            .font(SANS_SEMIBOLD)
            .style(move |_| iced::widget::text::Style { color: Some(label_c) }),
        text(value)
            .size(14)
            .font(MONO_SEMIBOLD)
            .style(move |_| iced::widget::text::Style { color: Some(value_color) }),
    ]
    .spacing(3)
    .align_x(iced::alignment::Horizontal::Right)
    .width(width)
    .into()
}

fn version_column(version: String, software: Option<String>, dark: bool) -> Element<'static, ResultsListMessage> {
    let label_c   = if dark { c("#5C636F") } else { c("#A0A7B1") };
    let ver_c     = if dark { c("#A2ABBA") } else { c("#3A4049") };
    let badge_bg  = if dark { c("#1F2630") } else { c("#EEF0F3") };
    let badge_brd = if dark { c("#2A3240") } else { c("#DDE2E8") };
    let badge_txt = if dark { c("#A2ABBA") } else { c("#3A4049") };

    let mut col = column![
        text("ВЕРСИЯ")
            .size(10)
            .font(SANS_SEMIBOLD)
            .style(move |_| iced::widget::text::Style { color: Some(label_c) }),
        text(version)
            .size(13)
            .font(MONO)
            .style(move |_| iced::widget::text::Style { color: Some(ver_c) }),
    ]
    .spacing(3)
    .align_x(iced::alignment::Horizontal::Right);

    if let Some(sw) = software {
        let badge = container(
            text(sw)
                .size(11)
                .font(MONO)
                .style(move |_| iced::widget::text::Style { color: Some(badge_txt) }),
        )
        .style(move |_: &_| ContainerStyle {
            background: Some(Background::Color(badge_bg)),
            border: Border { color: badge_brd, width: 1.0, radius: 4.0.into() },
            ..Default::default()
        })
        .padding(Padding::from([2, 6]));
        col = col.push(badge);
    }

    col.width(Fixed(150.0)).into()
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

fn avatar_palette(name: &str) -> (Color, Color, Color) {
    let first = name.chars().find(|c| c.is_alphanumeric()).unwrap_or('A') as u32;
    let idx = ((first.wrapping_mul(2654435769)) >> 28) as usize;

    const PALETTES: &[(u32, u32, u32)] = &[
        (0x214a3a, 0x16302a, 0x3DD68C),
        (0x33405c, 0x222a3d, 0x8FB3FF),
        (0x4a3a28, 0x2f2418, 0xE0B27A),
        (0x3d2a4a, 0x2a1d35, 0xC07AE0),
        (0x4a2828, 0x301818, 0xE07A7A),
        (0x1a3a4a, 0x11252f, 0x7AD4E0),
        (0x2d3d1a, 0x1d2811, 0xA3D97A),
        (0x4a3828, 0x302415, 0xE0C07A),
        (0x28384a, 0x18242f, 0x7AB8E0),
        (0x3a2a3a, 0x251825, 0xE07AC0),
        (0x1a3a3a, 0x112525, 0x7AE0D4),
        (0x3a3a1a, 0x252511, 0xD4E07A),
        (0x3a1a1a, 0x251111, 0xE08C7A),
        (0x1a2a3a, 0x111a25, 0x7AAEE0),
        (0x2a3a2a, 0x182518, 0x9AE09A),
        (0x3a2a1a, 0x251a0f, 0xE0B07A),
    ];

    let (gs, ge, lc) = PALETTES[idx % PALETTES.len()];
    (hex_color(gs), hex_color(ge), hex_color(lc))
}

fn hex_color(hex: u32) -> Color {
    Color {
        r: ((hex >> 16) & 0xFF) as f32 / 255.0,
        g: ((hex >> 8) & 0xFF) as f32 / 255.0,
        b: (hex & 0xFF) as f32 / 255.0,
        a: 1.0,
    }
}

fn build_avatar<'a>(name: &str, edition: &Edition) -> Element<'a, ResultsListMessage> {
    let first = name.chars().find(|c| c.is_alphanumeric()).unwrap_or('?')
        .to_uppercase().next().unwrap_or('?');
    let (grad_start, grad_end, letter_color) = avatar_palette(name);
    let angle = std::f32::consts::PI * 0.75;

    let letter_bg = container(
        text(first.to_string())
            .size(22)
            .font(SANS_SEMIBOLD)
            .style(move |_| iced::widget::text::Style { color: Some(letter_color) }),
    )
    .style(move |_: &_| ContainerStyle {
        background: Some(Background::Gradient(
            gradient::Linear::new(angle)
                .add_stop(0.0, grad_start)
                .add_stop(1.0, grad_end)
                .into(),
        )),
        border: Border { color: Color { r: 0.137, g: 0.165, b: 0.204, a: 1.0 }, width: 1.0, radius: 8.0.into() },
        ..Default::default()
    })
    .center(Fixed(52.0));

    let (badge_bg, badge_text_col, badge_letter) = match edition {
        Edition::Java    => (c("#D99A3C"), c("#08110B"), "J"),
        Edition::Bedrock => (c("#13A884"), c("#04120E"), "B"),
    };
    let border_col = c("#181D25");

    let badge_inner = container(
        container(
            text(badge_letter)
                .size(9)
                .font(MONO_SEMIBOLD)
                .style(move |_| iced::widget::text::Style { color: Some(badge_text_col) }),
        )
        .style(move |_: &_| ContainerStyle {
            background: Some(Background::Color(badge_bg)),
            border: Border { color: border_col, width: 2.0, radius: 5.0.into() },
            ..Default::default()
        })
        .center(Fixed(18.0)),
    )
    .width(Fixed(56.0))
    .height(Fixed(56.0))
    .align_right(Fixed(56.0))
    .align_bottom(Fixed(56.0));

    let avatar_layer = container(letter_bg)
        .width(Fixed(56.0))
        .height(Fixed(56.0))
        .align_x(iced::alignment::Horizontal::Left)
        .align_y(iced::alignment::Vertical::Top);

    Stack::new()
        .push(avatar_layer)
        .push(badge_inner)
        .width(Fixed(52.0))
        .height(Fixed(52.0))
        .into()
}

fn split_motd(motd: &str) -> (String, String) {
    let stripped = motd.trim();
    if let Some((first, rest)) = stripped.split_once('\n') {
        (first.trim().to_string(), rest.trim().to_string())
    } else {
        (stripped.to_string(), String::new())
    }
}
