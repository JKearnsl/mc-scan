mod styles;
mod components;

use crate::styles::{button_style, scrollable_style};
use iced::widget::{button, column, container, horizontal_space, image, row, scrollable, text, vertical_space};
use iced::{window, Alignment, Element, Fill, Padding, Shrink, Size, Task, Theme};
use iced::border::width;
use iced::window::icon;
use iced_aw::iced_fonts::required::icon_to_char;
use crate::components::address_list::{AddressList, AddressListMessage};

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

        let switch_theme_button = button("S")
            .style(button_style)
            .width(Shrink)
            .on_press(Message::SwitchTheme(match self.theme {
                Theme::Dark => Theme::Light,
                _ => Theme::Dark,
            }));

        let right_side = container(
            column![
                address_list,
                row![scan_button, horizontal_space(), switch_theme_button]
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
            .center_x(Fill)
            .center_y(Fill);

        // let current_theme = text(format!("Current theme: {}", self.theme));
        // MAIN ------------------------------------------------------------------------------------
        container(
            row![
                servers_list.width(iced::Length::FillPortion(70)),
                right_side.width(iced::Length::FillPortion(30))
            ]
                .width(Fill)
                .height(Fill)
                .spacing(10)
        )
            .padding(Padding {
                top: 0.0,
                right: 10.0,
                bottom: 0.0,
                left: 10.0,
            })
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
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