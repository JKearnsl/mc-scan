use iced::widget::scrollable::{default, Direction, Rail, Scroller, Scrollbar, Status, Style};
use iced::widget::{scrollable, Scrollable};
use iced::{border, Background, Element, Fill, Theme};

use crate::styles::{c, is_dark};

pub fn scrollbar<'a, M: Clone + 'a>(content: impl Into<Element<'a, M>>) -> Scrollable<'a, M> {
    scrollable(content)
        .style(style)
        .width(Fill)
        .height(Fill)
        .direction(Direction::Vertical(
            Scrollbar::new().width(4.0).margin(6.0).scroller_width(4.0),
        ))
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
