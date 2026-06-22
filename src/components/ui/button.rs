use iced::widget::{button, container, svg, text};
use iced::Length::Fixed;
use iced::{Background, Border, ContentFit, Element, Fill, Padding, Shadow, Theme};

use crate::styles::{button_danger, button_primary, c, is_dark, SANS_SEMIBOLD};

pub enum BtnVariant<'a> {
    Primary(&'a str),
    Secondary(&'a str),
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

        BtnVariant::Secondary(label) => button(
            container(text(label).size(14).font(SANS_SEMIBOLD)).center(Fill),
        )
        .on_press(on_press)
        .style(secondary_style)
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
            .style(icon_style)
            .padding(Padding::from([padding, padding]))
            .width(Fixed(btn_size))
            .height(Fixed(btn_size))
            .into()
        }
    }
}

fn secondary_style(t: &Theme, status: button::Status) -> button::Style {
    let dark = is_dark(t);
    let bg       = if dark { c("#181D25") } else { c("#FFFFFF") };
    let bg_hov   = if dark { c("#1F2630") } else { c("#F2F4F7") };
    let bg_prs   = if dark { c("#232A34") } else { c("#E8ECF0") };
    let text     = if dark { c("#A2ABBA") } else { c("#3A4049") };
    let text_hov = if dark { c("#E8EBF0") } else { c("#161A20") };
    let border_c = if dark { c("#232A34") } else { c("#DDE2E8") };

    let base = button::Style {
        background: Some(Background::Color(bg)),
        text_color: text,
        border: Border { color: border_c, width: 1.0, radius: 10.0.into() },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        button::Status::Hovered => button::Style { background: Some(Background::Color(bg_hov)), text_color: text_hov, ..base },
        button::Status::Pressed => button::Style { background: Some(Background::Color(bg_prs)), ..base },
        _ => base,
    }
}

fn icon_style(t: &Theme, status: button::Status) -> button::Style {
    let dark = is_dark(t);
    let bg       = if dark { c("#181D25") } else { c("#F6F7F9") };
    let bg_hov   = if dark { c("#1F2630") } else { c("#EEF0F3") };
    let bg_prs   = if dark { c("#232A34") } else { c("#E5E8EC") };
    let text     = if dark { c("#6B7480") } else { c("#5B6470") };
    let text_hov = if dark { c("#E8EBF0") } else { c("#161A20") };
    let border_c = if dark { c("#232A34") } else { c("#E1E5EA") };

    let base = button::Style {
        background: Some(Background::Color(bg)),
        text_color: text,
        border: Border { color: border_c, width: 1.0, radius: 10.0.into() },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        button::Status::Hovered => button::Style { background: Some(Background::Color(bg_hov)), text_color: text_hov, ..base },
        button::Status::Pressed => button::Style { background: Some(Background::Color(bg_prs)), ..base },
        _ => base,
    }
}
