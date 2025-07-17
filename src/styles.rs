use iced::border::Radius;
use iced::widget::button::{Status as ButtonStatus, Style as ButtonStyle};
use iced::widget::scrollable::{default as ScrollableStyleDefault, Rail, Scroller, Status as ScrollableStatus, Status, Style as ScrollableStyle};
use iced::{border, Background, Border, Color, Shadow, Theme, Vector};


pub fn button_style(theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    match status {
        ButtonStatus::Active |ButtonStatus::Pressed => match theme {
            Theme::Dark => {
                ButtonStyle {
                    background: Option::from(Background::Color(Color::parse("#2E2E2E").unwrap())),
                    text_color:  Color::parse("#ffffff").unwrap(),
                    border: Border {
                        color: Color::parse("#949494").unwrap(),
                        width: 0.0,
                        radius: Radius::from(5),
                    },
                    shadow: Shadow {
                        color: Color::parse("#949494").unwrap(),
                        offset: Vector::new(0.0, 0.0),
                        blur_radius: 3.0,
                    },
                }
            }
            _ => {
                ButtonStyle {
                    background: Option::from(Background::Color(Color::parse("#ffffff").unwrap())),
                    text_color:  Color::parse("#2E2E2E").unwrap(),
                    border: Border {
                        color: Color::parse("#ececec").unwrap(),
                        width: 0.5,
                        radius: Radius::from(5),
                    },
                    shadow: Shadow {
                        color: Color::parse("#C9A798").unwrap(),
                        offset: Vector::new(0.0, 0.0),
                        blur_radius: 3.0,
                    },
                }
            }
        },
        ButtonStatus::Hovered => match theme {
            Theme::Dark => {
                ButtonStyle {
                    background: Option::from(Background::Color(Color::parse("#9e9e9e").unwrap())),
                    text_color:  Color::parse("#ffffff").unwrap(),
                    border: Border {
                        color: Color::parse("#ececec").unwrap(),
                        width: 0.0,
                        radius: Radius::from(5),
                    },
                    shadow: Shadow {
                        color: Color::parse("#C9A798").unwrap(),
                        offset: Vector::new(0.0, 0.0),
                        blur_radius: 3.0,
                    },
                }
            }
            _ => {
                ButtonStyle {
                    background: Option::from(Background::Color(Color::parse("#ececec").unwrap())),
                    text_color:  Color::parse("#2E2E2E").unwrap(),
                    border: Border {
                        color: Color::parse("#ececec").unwrap(),
                        width: 0.5,
                        radius: Radius::from(5),
                    },
                    shadow: Shadow {
                        color: Color::parse("#C9A798").unwrap(),
                        offset: Vector::new(0.0, 0.0),
                        blur_radius: 3.0,
                    },
                }
            }
        },
        ButtonStatus::Disabled => match theme {
            Theme::Dark => {
                ButtonStyle {
                    background: Option::from(Background::Color(Color::parse("#2E2E2E").unwrap())),
                    text_color:  Color::parse("#9e9e9e").unwrap(),
                    border: Border {
                        color: Color::parse("#ececec").unwrap(),
                        width: 0.0,
                        radius: Radius::from(5),
                    },
                    shadow: Shadow {
                        color: Color::parse("#C9A798").unwrap(),
                        offset: Vector::new(0.0, 0.0),
                        blur_radius: 3.0,
                    },
                }
            }
            _ => {
                ButtonStyle {
                    background: Option::from(Background::Color(Color::parse("#ffffff").unwrap())),
                    text_color:  Color::parse("#9e9e9e").unwrap(),
                    border: Border {
                        color: Color::parse("#ececec").unwrap(),
                        width: 0.5,
                        radius: Radius::from(5),
                    },
                    shadow: Shadow {
                        color: Color::parse("#C9A798").unwrap(),
                        offset: Vector::new(0.0, 0.0),
                        blur_radius: 3.0,
                    },
                }
            }
        },
    }
}


pub fn scrollable_style(theme: &Theme, status: ScrollableStatus) -> ScrollableStyle {
    let palette = theme.extended_palette();

    match status {
        Status::Hovered { .. } | Status::Dragged { .. } => ScrollableStyle {
            vertical_rail: Rail {
                background: Some(palette.background.weak.color.into()),
                border: border::rounded(2),
                scroller: Scroller {
                    color: Color::parse("#9e9e9e").unwrap(),
                    border: border::rounded(2),
                },
            },
            ..ScrollableStyleDefault(theme, status)
        },
        _ => ScrollableStyleDefault(theme, status)
    }
}
