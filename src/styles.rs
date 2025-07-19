use iced::border::Radius;
use iced::theme::Palette;
use iced::widget::button::{Status as ButtonStatus, Style as ButtonStyle};
use iced::widget::container::Style as ContainerStyle;
use iced::widget::scrollable::{default as ScrollableStyleDefault, Rail, Scroller, Status as ScrollableStatus, Status, Style as ScrollableStyle};
use iced::{border, Background, Border, Color, Shadow, Theme, Vector};
use once_cell::sync::Lazy;

pub fn button_style(theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    match status {
        ButtonStatus::Active |ButtonStatus::Pressed => ButtonStyle {
            background: Option::from(Background::Color(Color::parse("#e3dca5").unwrap())),
            text_color:  Color::parse("#704012").unwrap(),
            border: Border {
                color: Color::parse("#e7e0b0").unwrap(),
                width: 0.5,
                radius: Radius::from(5),
            },
            shadow: Shadow {
                color: Color::parse("#e7e0b0").unwrap(),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 1.0,
            },
        },
        ButtonStatus::Hovered => ButtonStyle {
            background: Option::from(Background::Color(Color::parse("#e7e0b0").unwrap())),
            text_color:  Color::parse("#2E2E2E").unwrap(),
            border: Border {
                color: Color::parse("#e7e0b0").unwrap(),
                width: 0.5,
                radius: Radius::from(5),
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        },
        ButtonStatus::Disabled => ButtonStyle {
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
        },
    }
}

pub fn icon_button_style(_: &Theme, status: ButtonStatus) -> ButtonStyle {
    match status {
        _ => ButtonStyle {
            background: None,
            text_color:  Color::TRANSPARENT,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(15),
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        },
    }
}


pub fn scrollable_style(theme: &Theme, status: ScrollableStatus) -> ScrollableStyle {
    let palette = theme.extended_palette();

    match status {
        Status::Active => ScrollableStyle {
            vertical_rail: Rail {
                background: Some(palette.secondary.weak.color.into()),
                border: border::rounded(2),
                scroller: Scroller {
                    color: Color::parse("#ddd48f").unwrap(),
                    border: border::rounded(2),
                },
            },
            ..ScrollableStyleDefault(theme, status)
        },
        Status::Hovered { .. } | Status::Dragged { .. } => ScrollableStyle {
            vertical_rail: Rail {
                background: Some(palette.secondary.weak.color.into()),
                border: border::rounded(2),
                scroller: Scroller {
                    color: Color::parse("#e7e0b0").unwrap(),
                    border: border::rounded(2),
                },
            },
            ..ScrollableStyleDefault(theme, status)
        },
    }
}

pub fn right_side_style(_: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Option::from(Background::Color(Color::parse("#cda989").unwrap())),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(0),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
        ..ContainerStyle::default()
    }
}

pub static COLOR_THEME: Lazy<Theme> = Lazy::new(|| Theme::custom(
    "avocado".into(),
    Palette {
        background: Color::parse("#b3c656").unwrap(),
        text: Color::parse("#e3dca5").unwrap(),
        primary: Color::parse("#6b8c21").unwrap(),
        success: Color::parse("#6b8c21").unwrap(),
        danger: Color::parse("#dc3545").unwrap(),
    }
));