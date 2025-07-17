use crate::styles::scrollable_style;
use iced::border::Radius;
use iced::widget::button::{Status as ButtonStatus, Style as ButtonStyle};
use iced::widget::container::Style as ContainerStyle;
use iced::widget::text_input::Status;
use iced::widget::text_input::Style as TextInputStyle;
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text, text_input};
use iced::{Background, Border, Center, Color, Element, Fill, Padding, Shadow, Shrink, Theme, Vector};
use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use std::net::IpAddr;
use std::str::FromStr;

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

        let add_button = button("Add")
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

        for (i, ip_net) in self.values.iter().enumerate() {
            addresses = addresses.push(
                row![
                    text(match ip_net {
                        IpNet::V4(net) => format!("{}", net),
                        IpNet::V6(net) => format!("{}", net),
                    }),
                    horizontal_space(),
                    button("-")
                        .on_press(AddressListMessage::RemoveClicked(i))
                        .style(add_button_style)
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
        ]
            .spacing(1)
        )
            .style(style)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }
}



pub fn style(theme: &Theme) -> ContainerStyle {
    match theme {
        Theme::Dark => {
            ContainerStyle {
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
                ..ContainerStyle::default()
            }
        }
        _ => {
            ContainerStyle {
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
                ..ContainerStyle::default()
            }
        }
    }
}

pub fn input_style(theme: &Theme, _: Status) -> TextInputStyle {
    match theme {
        Theme::Dark => TextInputStyle {
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                radius: 5.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            icon: Color::TRANSPARENT,
            placeholder: Color::parse("#4f4f4f").unwrap(),
            value: Color::WHITE,
            selection: Color::parse("#4f4f4f").unwrap(),
        },
        _ => TextInputStyle {
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                radius: 5.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            icon: Color::TRANSPARENT,
            placeholder: Color::parse("#dbdbdb").unwrap(),
            value: Color::BLACK,
            selection: Color::parse("#dbdbdb").unwrap(),
        }
    }
}

pub fn input_style_err(theme: &Theme, _: Status) -> TextInputStyle {
    match theme {
        Theme::Dark => TextInputStyle {
            value: Color::parse("#d12e2e").unwrap(),
            ..input_style(theme, Status::Active)
        },
        _ => TextInputStyle {
            value: Color::parse("#ff0000").unwrap(),
            ..input_style(theme, Status::Active)
        }
    }
}

pub fn input_container_style(theme: &Theme) -> ContainerStyle {
    match theme {
        Theme::Dark => {
            ContainerStyle {
                border: Border {
                    color: Color::parse("#949494").unwrap(),
                    width: 0.5,
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
        _ => {
            ContainerStyle {
                border: Border {
                    color: Color::parse("#ececec").unwrap(),
                    width: 1.0,
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
    }
}

pub fn add_button_style(theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let active = match theme {
        Theme::Dark => ButtonStyle {
            background: Option::from(Background::Color(Color::parse("#2E2E2E").unwrap())),
            text_color:  Color::parse("#ffffff").unwrap(),
            border: Border {
                color: Color::parse("#949494").unwrap(),
                width: 0.5,
                radius: Radius::from(5),
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        },
        _ => ButtonStyle {
            background: Option::from(Background::Color(Color::parse("#ffffff").unwrap())),
            text_color:  Color::parse("#2E2E2E").unwrap(),
            border: Border {
                color: Color::parse("#ececec").unwrap(),
                width: 1.0,
                radius: Radius::from(5),
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        }
    };

    match status {
        ButtonStatus::Active |ButtonStatus::Pressed => active.clone(),
        ButtonStatus::Hovered => match theme {
            Theme::Dark => ButtonStyle {
                background: Option::from(Background::Color(Color::parse("#9e9e9e").unwrap())),
                text_color:  Color::parse("#ffffff").unwrap(),
                ..active
            },
            _ => ButtonStyle {
                background: Option::from(Background::Color(Color::parse("#ececec").unwrap())),
                text_color:  Color::parse("#2E2E2E").unwrap(),
                ..active
            }
        },
        ButtonStatus::Disabled => match theme {
            Theme::Dark => ButtonStyle {
                background: Option::from(Background::Color(Color::parse("#2E2E2E").unwrap())),
                text_color:  Color::parse("#9e9e9e").unwrap(),
                ..active
            },
            _ => ButtonStyle {
                background: Option::from(Background::Color(Color::parse("#ffffff").unwrap())),
                text_color:  Color::parse("#9e9e9e").unwrap(),
                ..active
            }
        },
    }
}
