use iced::widget::{button, container, svg, text};
use iced::Length::Fixed;
use iced::{ContentFit, Element, Fill, Padding, Theme};

use crate::styles::{button_danger, button_primary, c, icon_button_style, is_dark, SANS_SEMIBOLD};

pub enum BtnVariant<'a> {
    Primary(&'a str),
    Danger(&'a str),
    Icon { handle: svg::Handle, size: f32 },
}

pub fn btn<'a, M: Clone + 'a>(variant: BtnVariant<'a>, on_press: M) -> Element<'a, M> {
    match variant {
        BtnVariant::Primary(label) => button(
            container(text(label).size(14).font(SANS_SEMIBOLD)).center(Fill),
        )
        .on_press(on_press)
        .style(button_primary)
        .width(Fill)
        .height(Fixed(44.0))
        .into(),

        BtnVariant::Danger(label) => button(
            container(text(label).size(14).font(SANS_SEMIBOLD)).center(Fill),
        )
        .on_press(on_press)
        .style(button_danger)
        .width(Fill)
        .height(Fixed(44.0))
        .into(),

        BtnVariant::Icon { handle, size } => {
            let padding = (size * 0.7).round() as u16;
            let btn_size = size + (padding * 2) as f32;
            button(
                svg(handle)
                    .content_fit(ContentFit::Fill)
                    .width(Fixed(size))
                    .height(Fixed(size))
                    .style(|t: &Theme, _| svg::Style {
                        color: Some(if is_dark(t) { c("#6B7480") } else { c("#5B6470") }),
                    }),
            )
            .on_press(on_press)
            .style(icon_button_style)
            .padding(Padding::from([padding, padding]))
            .width(Fixed(btn_size))
            .height(Fixed(btn_size))
            .into()
        }
    }
}
