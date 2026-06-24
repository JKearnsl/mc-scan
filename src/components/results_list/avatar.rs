use iced::widget::container::Style as ContainerStyle;
use iced::widget::{container, text, Stack};
use iced::Length::Fixed;
use iced::{gradient, Background, Border, Color, Element, Theme};

use crate::scanner::types::Edition;
use crate::styles::{c, is_dark, MONO_SEMIBOLD, SANS_SEMIBOLD};

use super::ResultsListMessage;

pub fn build_avatar<'a>(name: &str, edition: &Edition) -> Element<'a, ResultsListMessage> {
    let first = name.chars().find(|c| c.is_alphanumeric()).unwrap_or('?')
        .to_uppercase().next().unwrap_or('?');
    let (grad_start, grad_end, letter_color) = palette(name);
    let angle = std::f32::consts::PI * 0.75;

    let letter_bg = container(
        text(first.to_string())
            .size(22)
            .font(SANS_SEMIBOLD)
            .style(move |_: &Theme| text::Style { color: Some(letter_color) }),
    )
    .style(move |_: &Theme| ContainerStyle {
        background: Some(Background::Gradient(
            gradient::Linear::new(angle)
                .add_stop(0.0, grad_start)
                .add_stop(1.0, grad_end)
                .into(),
        )),
        border: Border {
            color: Color::TRANSPARENT,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .center(Fixed(52.0));

    let (badge_bg, badge_text_col, badge_letter) = match edition {
        Edition::Java    => (c("#D99A3C"), c("#08110B"), "J"),
        Edition::Bedrock => (c("#13A884"), c("#04120E"), "B"),
    };

    let badge_inner = container(
        container(
            text(badge_letter)
                .size(9)
                .font(MONO_SEMIBOLD)
                .style(move |_: &Theme| text::Style { color: Some(badge_text_col) }),
        )
        .style(move |t: &Theme| ContainerStyle {
            background: Some(Background::Color(badge_bg)),
            border: Border {
                color: if is_dark(t) { c("#181D25") } else { c("#FFFFFF") },
                width: 2.0,
                radius: 5.0.into(),
            },
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
        .width(Fixed(56.0))
        .height(Fixed(56.0))
        .into()
}

fn palette(name: &str) -> (Color, Color, Color) {
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
    (from_hex(gs), from_hex(ge), from_hex(lc))
}

fn from_hex(hex: u32) -> Color {
    Color {
        r: ((hex >> 16) & 0xFF) as f32 / 255.0,
        g: ((hex >> 8) & 0xFF) as f32 / 255.0,
        b: (hex & 0xFF) as f32 / 255.0,
        a: 1.0,
    }
}
