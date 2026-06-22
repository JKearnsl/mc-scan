use iced::widget::scrollable::{default, Rail, Scroller, Status, Style};
use iced::widget::{scrollable, Scrollable};
use iced::{border, Background, Element, Fill, Theme};

use crate::styles::{c, is_dark};

pub fn scrollbar<'a, M: Clone + 'a>(content: impl Into<Element<'a, M>>) -> Scrollable<'a, M> {
    scrollable(content)
        .style(style)
        .width(Fill)
        .height(Fill)
}

fn style(t: &Theme, status: Status) -> Style {
    let dark = is_dark(t);
    let scroller = Scroller {
        background: Background::Color(if dark { c("#232A34") } else { c("#C8CDD5") }),
        border: border::rounded(2),
    };
    let rail = Rail {
        background: Some(Background::Color(if dark { c("#1A1F27") } else { c("#E1E5EA") })),
        border: border::rounded(2),
        scroller,
    };
    Style {
        vertical_rail: rail,
        ..default(t, status)
    }
}
