use iced::widget::scrollable::{default as ScrollableStyleDefault, Rail, Scroller, Status as ScrollableStatus, Style as ScrollableStyle};
use iced::border::Radius;
use iced::widget::button::{Status as ButtonStatus, Style as ButtonStyle};
use iced::widget::container::Style as ContainerStyle;
use iced::widget::text_input::Style as TextInputStyle;
use iced::widget::text_input::Status as TextInputStatus;
use iced::widget::{button, column, container, horizontal_space, row, scrollable, svg, text, text_input};
use iced::{border, color, Background, Border, Center, Color, ContentFit, Element, Fill, Padding, Shadow, Shrink, Theme, Vector};
use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use std::net::IpAddr;
use std::str::FromStr;
use iced::Length::Fixed;
use crate::styles::icon_button_style;

#[derive(Debug, Clone)]
pub enum AddressListMessage {
    AddClicked,
    RemoveClicked(usize),
    InputChanged(String),
}

#[derive(Default)]
pub struct AddressList {
    input_address: String,
    input_error: bool,
    values: Vec<IpNet>,
}

impl AddressList {
    pub fn update(&mut self, message: AddressListMessage) {
        match message {
            AddressListMessage::AddClicked => match IpNet::from_str(&self.input_address) {
                Ok(ip_net) => {
                    self.values.push(ip_net);
                    self.input_address.clear();
                }
                Err(_) => {
                    if let Ok(ip_addr) = self.input_address.parse::<IpAddr>() {
                        let ip_net = match ip_addr {
                            IpAddr::V4(addr) => IpNet::V4(Ipv4Net::new(addr, 32).unwrap()),
                            IpAddr::V6(addr) => IpNet::V6(Ipv6Net::new(addr, 128).unwrap()),
                        };
                        self.values.push(ip_net);
                        self.input_address.clear();
                    } else {
                        self.input_error = true;
                    }
                }
            },
            AddressListMessage::InputChanged(text) => {
                self.input_error = false;
                self.input_address = text
            },
            AddressListMessage::RemoveClicked(index) => {
                self.values.remove(index);
            },
        }
    }

    pub fn view(&'_ self) -> Element<'_, AddressListMessage> {
        let mut address_input = text_input("0.0.0.0/0", &self.input_address)
            .on_input(AddressListMessage::InputChanged)
            .padding(Padding {
                top: 5.0,
                right: 10.0,
                bottom: 5.0,
                left: 10.0,
            })
            .style(input_style)
            .width(Fill);

        if self.input_error {
            address_input = address_input.style(input_style_err);
        }

        let add_button = button("+")
            .on_press(AddressListMessage::AddClicked)
            .padding(Padding {
                top: 5.0,
                right: 10.0,
                bottom: 5.0,
                left: 10.0,
            })
            .style(add_button_style)
            .width(Shrink);

        let mut addresses = iced::widget::column![]
            .padding(Padding {
                top: 5.0,
                right: 0.0,
                bottom: 5.0,
                left: 10.0,
            }).spacing(5);

        let handle = svg::Handle::from_path(format!(
            "{}/assets/trash.svg",
            env!("CARGO_MANIFEST_DIR")
        ));

        for (i, ip_net) in self.values.iter().enumerate() {
            addresses = addresses.push(
                row![
                    text(match ip_net {
                        IpNet::V4(net) => format!("{}", net),
                        IpNet::V6(net) => format!("{}", net),
                    }),
                    horizontal_space(),
                    button(svg(handle.clone())
                        .content_fit(ContentFit::Fill)
                        .width(Fill)
                        .height(Fill)
                        .style(|_theme, _status| svg::Style {
                            color: Some(color!(0x9f3838))
                        })
                    )
                        .style(icon_button_style)
                        .padding(Padding::ZERO)
                        .width(Fixed(24.0))
                        .height(Fixed(24.0))
                        .on_press(AddressListMessage::RemoveClicked(i))
                ]
                    .spacing(10)
                    .align_y(Center)
            );
        }

        container(column![
            container(row![address_input, add_button]).style(input_container_style),
            scrollable(addresses)
                .style(scrollable_style)
                .spacing(10)
                .width(Fill)
                .height(Fill)
        ])
            .style(style)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }
}



pub fn style(_: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Option::from(Background::Color(Color::parse("#d7baa0").unwrap())),
        border: Border {
            color: Color::parse("#a4876d").unwrap(),
            width: 5.0,
            radius: Radius::from(5),
        },
        shadow: Shadow {
            color: Color::parse("#a4876d").unwrap(),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 1.0,
        },
        text_color: Option::from(Color::parse("#665e1d").unwrap()),
        ..ContainerStyle::default()
    }
}

pub fn input_style(_: &Theme, _: TextInputStatus) -> TextInputStyle {
    TextInputStyle {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            radius: 5.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        icon: Color::TRANSPARENT,
        placeholder: Color::parse("#c0b7af").unwrap(),
        value: Color::parse("#e3dca5").unwrap(),
        selection: Color::parse("#c0b7af").unwrap(),
    }
}

pub fn input_style_err(theme: &Theme, _: TextInputStatus) -> TextInputStyle {
    TextInputStyle {
        value: Color::parse("#990000").unwrap(),
        ..input_style(theme, TextInputStatus::Active)
    }
}

pub fn input_container_style(_: &Theme) -> ContainerStyle {
    ContainerStyle {
        background: Option::from(Background::Color(Color::parse("#a4876d").unwrap())),
        border: Border {
            color: Color::parse("#a4876d").unwrap(),
            width: 5.0,
            radius: Radius::from(5),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
        ..ContainerStyle::default()
    }
}

pub fn add_button_style(_: &Theme, status: ButtonStatus) -> ButtonStyle {
    let active = ButtonStyle {
        background: Option::from(Background::Color(Color::parse("#e3dca5").unwrap())),
        text_color:  Color::parse("#704012").unwrap(),
        border: Border {
            color: Color::TRANSPARENT,
            width: 5.0,
            radius: Radius::from(8),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
    };

    match status {
        ButtonStatus::Active |ButtonStatus::Pressed => active.clone(),
        ButtonStatus::Hovered => ButtonStyle {
            background: Option::from(Background::Color(Color::parse("#ececec").unwrap())),
            text_color:  Color::parse("#2E2E2E").unwrap(),
            ..active
        },
        ButtonStatus::Disabled => ButtonStyle {
            background: Option::from(Background::Color(Color::parse("#ffffff").unwrap())),
            text_color:  Color::parse("#9e9e9e").unwrap(),
            ..active
        },
    }
}

pub fn scrollable_style(theme: &Theme, status: ScrollableStatus) -> ScrollableStyle {
    match status {
        ScrollableStatus::Active => ScrollableStyle {
            vertical_rail: Rail {
                background: None,
                border: border::rounded(2),
                scroller: Scroller {
                    color: Color::TRANSPARENT,
                    border: border::rounded(2),
                },
            },
            ..ScrollableStyleDefault(theme, status)
        },
        _ => ScrollableStyle {
            vertical_rail: Rail {
                background: Some(Color::parse("#a4876d").unwrap().into()),
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
