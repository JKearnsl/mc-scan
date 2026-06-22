use iced::widget::text_editor;
use iced::Length::Fixed;
use iced::{Element, Padding, Theme};

use crate::styles::{c, is_dark, MONO};

pub fn textarea<'a, M: Clone + 'a>(
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> M + 'a,
    height: f32,
) -> Element<'a, M> {
    text_editor(content)
        .on_action(on_action)
        .height(Fixed(height))
        .style(style)
        .font(MONO)
        .size(13)
        .padding(Padding::from([8, 10]))
        .into()
}

fn style(t: &Theme, _: text_editor::Status) -> text_editor::Style {
    let dark = is_dark(t);
    text_editor::Style {
        background: iced::Background::Color(if dark { c("#181D25") } else { c("#FFFFFF") }),
        border: iced::Border {
            color: if dark { c("#232A34") } else { c("#DDE2E8") },
            width: 1.0,
            radius: 7.0.into(),
        },
        placeholder: if dark { c("#5C636F") } else { c("#A0A7B1") },
        value: if dark { c("#E8EBF0") } else { c("#161A20") },
        selection: iced::Color { r: 0.239, g: 0.839, b: 0.549, a: 0.25 },
    }
}
