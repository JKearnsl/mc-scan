use base64::Engine;
use iced::Length::Fixed;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::{Stack, container, image, text};
use iced::{Background, Border, Color, ContentFit, Element, Theme, gradient};

use crate::styles::{MONO_SEMIBOLD, SANS_SEMIBOLD, c, is_dark};
use scanner::types::Edition;

#[derive(Clone, Copy)]
pub struct AvatarSize {
    pub outer: f32,
    pub inner: f32,
    pub letter_font: f32,
    pub letter_radius: f32,
    pub badge_size: f32,
    pub badge_border: f32,
    pub badge_radius: f32,
    pub badge_font: f32,
}

impl AvatarSize {
    pub const SMALL: AvatarSize = AvatarSize {
        outer: 56.0,
        inner: 52.0,
        letter_font: 22.0,
        letter_radius: 8.0,
        badge_size: 18.0,
        badge_border: 2.0,
        badge_radius: 5.0,
        badge_font: 9.0,
    };

    pub const LARGE: AvatarSize = AvatarSize {
        outer: 72.0,
        inner: 68.0,
        letter_font: 27.0,
        letter_radius: 14.0,
        badge_size: 22.0,
        badge_border: 2.5,
        badge_radius: 6.0,
        badge_font: 11.0,
    };
}

pub fn build_avatar_icon<'a, M: 'a>(
    name: &str,
    edition: &Edition,
    favicon: Option<image::Handle>,
    size: AvatarSize,
    ring: impl Fn(&Theme) -> Color + 'a,
) -> Element<'a, M> {
    let first = name
        .chars()
        .find(|c| c.is_alphanumeric())
        .unwrap_or('?')
        .to_uppercase()
        .next()
        .unwrap_or('?');

    let (dark_start, dark_end, dark_letter) = palette(name);
    let (light_start, light_end, light_letter) = light_variant(dark_letter);
    let angle = std::f32::consts::PI * 0.75;

    let base_layer: Element<'a, M> = if let Some(handle) = favicon {
        container(
            image(handle)
                .content_fit(ContentFit::Contain)
                .width(Fixed(size.inner))
                .height(Fixed(size.inner)),
        )
        .style(move |t: &Theme| ContainerStyle {
            border: Border {
                color: if is_dark(t) {
                    c("#2A3240")
                } else {
                    c("#DDE2E8")
                },
                width: 1.0,
                radius: size.letter_radius.into(),
            },
            ..Default::default()
        })
        .center(Fixed(size.inner))
        .into()
    } else {
        container(
            text(first.to_string())
                .size(size.letter_font)
                .font(SANS_SEMIBOLD)
                .style(move |t: &Theme| text::Style {
                    color: Some(if is_dark(t) {
                        dark_letter
                    } else {
                        light_letter
                    }),
                }),
        )
        .style(move |t: &Theme| {
            let (gs, ge) = if is_dark(t) {
                (dark_start, dark_end)
            } else {
                (light_start, light_end)
            };
            ContainerStyle {
                background: Some(Background::Gradient(
                    gradient::Linear::new(angle)
                        .add_stop(0.0, gs)
                        .add_stop(1.0, ge)
                        .into(),
                )),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 1.0,
                    radius: size.letter_radius.into(),
                },
                ..Default::default()
            }
        })
        .center(Fixed(size.inner))
        .into()
    };

    let (badge_bg, badge_text_col, badge_letter) = match edition {
        Edition::Java => (c("#D99A3C"), c("#08110B"), "J"),
        Edition::Bedrock => (c("#13A884"), c("#04120E"), "B"),
    };

    let badge_fill = container(
        text(badge_letter)
            .size(size.badge_font)
            .font(MONO_SEMIBOLD)
            .style(move |t: &Theme| text::Style {
                color: Some(if is_dark(t) {
                    badge_text_col
                } else {
                    Color::WHITE
                }),
            }),
    )
    .style(move |_: &Theme| ContainerStyle {
        background: Some(Background::Color(badge_bg)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: (size.badge_radius - size.badge_border).max(0.0).into(),
        },
        ..Default::default()
    })
    .center(Fixed(size.badge_size - size.badge_border * 2.0));

    let badge_inner = container(
        container(badge_fill)
            .style(move |t: &Theme| ContainerStyle {
                background: Some(Background::Color(ring(t))),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: size.badge_radius.into(),
                },
                ..Default::default()
            })
            .center(Fixed(size.badge_size)),
    )
    .width(Fixed(size.outer))
    .height(Fixed(size.outer))
    .align_right(Fixed(size.outer))
    .align_bottom(Fixed(size.outer));

    let avatar_layer = container(base_layer)
        .width(Fixed(size.outer))
        .height(Fixed(size.outer))
        .align_x(iced::alignment::Horizontal::Left)
        .align_y(iced::alignment::Vertical::Top);

    Stack::new()
        .push(avatar_layer)
        .push(badge_inner)
        .width(Fixed(size.outer))
        .height(Fixed(size.outer))
        .into()
}

pub fn favicon_handle(favicon: &str, size: AvatarSize) -> Option<image::Handle> {
    rounded_favicon(favicon, size.inner, size.letter_radius)
}

fn decode_favicon(favicon: &str) -> Option<Vec<u8>> {
    let b64 = favicon.split(',').next_back()?.trim();
    if b64.is_empty() {
        return None;
    }
    base64::engine::general_purpose::STANDARD.decode(b64).ok()
}

fn rounded_favicon(favicon: &str, display: f32, display_radius: f32) -> Option<image::Handle> {
    use ::image::imageops::FilterType;

    let bytes = decode_favicon(favicon)?;

    // Bound the decode: a hostile favicon could be a PNG decompression bomb.
    let mut reader = ::image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .ok()?;
    let mut limits = ::image::Limits::default();
    limits.max_image_width = Some(1024);
    limits.max_image_height = Some(1024);
    limits.max_alloc = Some(16 * 1024 * 1024);
    reader.limits(limits);
    let img = reader.decode().ok()?;

    let (w, h) = (img.width(), img.height());
    let side = w.min(h);
    if side == 0 {
        return None;
    }
    let square = img.crop_imm((w - side) / 2, (h - side) / 2, side, side);

    let target = ((display.ceil() as u32).max(1)) * 2;
    let mut buf = square
        .resize_exact(target, target, FilterType::Lanczos3)
        .to_rgba8();

    let radius = (display_radius * target as f32 / display).clamp(0.0, target as f32 / 2.0);
    let (tw, th) = (target as f32, target as f32);
    for (x, y, px) in buf.enumerate_pixels_mut() {
        let px_c = x as f32 + 0.5;
        let py_c = y as f32 + 0.5;
        let dx = (radius - px_c).max(px_c - (tw - radius)).max(0.0);
        let dy = (radius - py_c).max(py_c - (th - radius)).max(0.0);
        let dist = (dx * dx + dy * dy).sqrt();
        let coverage = (radius - dist + 0.5).clamp(0.0, 1.0);
        px[3] = (px[3] as f32 * coverage).round() as u8;
    }

    Some(image::Handle::from_rgba(target, target, buf.into_raw()))
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

    const HEX_TO_COLOR: fn(u32) -> Color = |hex: u32| -> Color {
        Color {
            r: ((hex >> 16) & 0xFF) as f32 / 255.0,
            g: ((hex >> 8) & 0xFF) as f32 / 255.0,
            b: (hex & 0xFF) as f32 / 255.0,
            a: 1.0,
        }
    };

    let (gs, ge, lc) = PALETTES[idx % PALETTES.len()];
    (HEX_TO_COLOR(gs), HEX_TO_COLOR(ge), HEX_TO_COLOR(lc))
}

fn light_variant(dark_letter: Color) -> (Color, Color, Color) {
    let (h, s, _) = rgb_to_hsl(dark_letter);
    let bg_s = (s * 0.85).min(1.0);
    let bg_start = hsl_to_color(h, bg_s, 0.91);
    let bg_end = hsl_to_color(h, bg_s, 0.83);
    let letter = hsl_to_color(h, s.min(0.8), 0.37);
    (bg_start, bg_end, letter)
}

// Hue in turns (0..1).
fn rgb_to_hsl(c: Color) -> (f32, f32, f32) {
    let (r, g, b) = (c.r, c.g, c.b);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    let d = max - min;
    if d < 1e-6 {
        return (0.0, 0.0, l);
    }
    let s = d / (1.0 - (2.0 * l - 1.0).abs());
    let h = if max == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    (h / 6.0, s, l)
}

fn hsl_to_color(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = (h * 6.0).rem_euclid(6.0);
    let x = c * (1.0 - ((hp % 2.0) - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    Color {
        r: r + m,
        g: g + m,
        b: b + m,
        a: 1.0,
    }
}
