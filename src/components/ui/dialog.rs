use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{button, column, container, row, text};
use iced::Length::Fixed;
use iced::{Alignment, Background, Border, Color, Element, Fill, Padding, Theme};

use crate::styles::{c, icon_button_style, is_dark, SANS_SEMIBOLD};


pub fn dialog<'a, M: Clone + 'a>(
    title: &'a str,
    on_close: M,
    width: f32,
    body: Element<'a, M>,
) -> Element<'a, M> {
    let inner = column![
        row![
            text(title).size(16).font(SANS_SEMIBOLD).style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) { c("#E8EBF0") } else { c("#161A20") }),
            }),
            Space::new().width(Fill),
            button("×")
                .on_press(on_close)
                .style(icon_button_style)
                .padding(Padding::from([4, 10])),
        ]
        .align_y(Alignment::Center),
        body
    ]
        .padding(Padding::from([20, 24]));

    let dlg = container(inner).width(Fixed(width)).style(style);

    container(dlg)
        .center_x(Fill)
        .center_y(Fill)
        .style(|_: &Theme| ContainerStyle {
            background: Some(Background::Color(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.70 })),
            ..Default::default()
        })
        .into()
}

fn style(t: &Theme) -> ContainerStyle {
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
