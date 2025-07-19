mod styles;
mod components;

use crate::components::address_list::{AddressList, AddressListMessage};
use crate::styles::{button_style, icon_button_style, right_side_style, scrollable_style, COLOR_THEME};
use iced::widget::{button, column, container, horizontal_space, row, scrollable, svg, vertical_space};
use iced::Length::Fixed;
use iced::{color, window, Alignment, ContentFit, Element, Fill, Padding, Size, Task, Theme};

#[derive(Default)]
struct McScan {
    pub wid: Option<window::Id>,
    theme: Theme,
    value: i64,
    address_list: AddressList
}


#[derive(Debug, Clone)]
pub enum Message {
    WindowInitialized(Option<window::Id>),
    Scan(Option<u64>),
    AddressList(AddressListMessage),
    SwitchTheme(Theme),
    Settings,
}

impl McScan {

    fn init() -> (Self, Task<Message>) {
        (
            Self::default(),
            Task::discard(window::get_latest()).map(Message::WindowInitialized)
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::WindowInitialized(id) => {
                self.wid = id;
                // todo setup window size
            }
            Message::Scan(id) => {
                self.value -= 1;
            },
            Message::Settings => {
                // todo open settings
            },
            Message::SwitchTheme(theme) => match theme {
                Theme::Dark => {
                    self.theme = Theme::Dark;
                }
                _ => {
                    self.theme = Theme::Light;
                }
            },
            Message::AddressList(msg) => self.address_list.update(msg)
        }
    }



    fn view(&'_ self) -> Element<'_, Message> {
        // LEFT SIDE -------------------------------------------------------------------------------
        let servers_list =  container(
            scrollable(column![
                "Scroll me!",
                vertical_space().height(3000),
                "You did it!",
            ])
            .width(Fill)
            .height(Fill)
            .style(scrollable_style)
        )
            .center_x(Fill)
            .center_y(Fill);

        // RIGHT SIDE ------------------------------------------------------------------------------
        let scan_button = button("Scan")
            .style(button_style)
            .on_press(Message::Scan(None)
        );

        let address_list = self.address_list.view().map(Message::AddressList);
        let handle = svg::Handle::from_path(format!(
            "{}/assets/settings.svg",
            env!("CARGO_MANIFEST_DIR")
        ));



        let open_settings_button = button(svg(handle)
            .content_fit(ContentFit::Fill)
            .width(Fill)
            .height(Fill)
            .style(|_theme, _status| svg::Style {
                color: Some(color!(0xe3dca5))
            })
        )
            .style(icon_button_style)
            .padding(Padding::ZERO)
            .width(Fixed(24.0))
            .height(Fixed(24.0))
            .on_press(Message::SwitchTheme(match self.theme {
                Theme::Dark => Theme::Light,
                _ => Theme::Dark,
            }));

        let right_side = container(
            column![
                row![scan_button, horizontal_space(), open_settings_button]
                    .align_y(Alignment::Center),
                address_list,
            ]
                .spacing(10)
                .padding(Padding {
                    top: 10.0,
                    right: 5.0,
                    bottom: 10.0,
                    left: 5.0,
                })
                .align_x(Alignment::Center)
                .width(Fill)
                .height(Fill)
        )
            .style(right_side_style)
            .padding(Padding {
                top: 0.0,
                right: 10.0,
                bottom: 0.0,
                left: 10.0,
            })
            .center_x(Fill)
            .center_y(Fill);

        // MAIN ------------------------------------------------------------------------------------
        container(
            row![
                servers_list.width(iced::Length::FillPortion(70)),
                right_side.width(iced::Length::FillPortion(30))
            ]
                .width(Fill)
                .height(Fill)
        )
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        COLOR_THEME.clone()
    }
}


fn main() -> iced::Result {
    iced::application("mc-scan", McScan::update, McScan::view)
        .theme(McScan::theme)
        .window(window::Settings {
            size: Size {
                width: 670.0,
                height: 370.0
            },
            min_size: Some(Size {
                width: 640.0,
                height: 340.0,
            }),
            max_size: Some(Size {
                width: 800.0,
                height: 600.0,
            }),
            resizable: true,
            ..window::Settings::default()
        })
        .run_with(McScan::init)
}