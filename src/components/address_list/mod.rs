pub mod add_dialog;

use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{button, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Background, Border, Color, Element, Fill, Padding, Theme};
use iced::Length::Fixed;
use ipnet::IpNet;
use crate::styles::{c, is_dark, scrollable_style, MONO};

#[derive(Debug, Clone)]
pub enum AddressListMessage {
    RemoveClicked(usize),
    HoverEnter(usize),
    HoverExit,
    ScrollChanged(bool),
}

pub struct AddressList {
    values: Vec<IpNet>,
    hover_index: Option<usize>,
    is_scrollable: bool,
}

impl Default for AddressList {
    fn default() -> Self {
        Self { values: Vec::new(), hover_index: None, is_scrollable: false }
    }
}

impl AddressList {
    pub fn values(&self) -> &[IpNet] {
        &self.values
    }

    pub fn push_ranges(&mut self, ranges: Vec<IpNet>) {
        for r in ranges {
            if !self.values.contains(&r) {
                self.values.push(r);
            }
        }
    }

    pub fn update(&mut self, message: AddressListMessage) {
        match message {
            AddressListMessage::RemoveClicked(i) => {
                self.values.remove(i);
                self.hover_index = None;
            }
            AddressListMessage::HoverEnter(i) => self.hover_index = Some(i),
            AddressListMessage::HoverExit => self.hover_index = None,
            AddressListMessage::ScrollChanged(scrollable) => {
                self.is_scrollable = scrollable;
            }
        }
    }

    pub fn total_hosts(&self) -> u64 {
        self.values.iter().map(net_host_count).sum()
    }

    pub fn view(&self) -> Element<'_, AddressListMessage> {
        let trash_handle = svg::Handle::from_path(format!(
            "{}/assets/trash.svg",
            env!("CARGO_MANIFEST_DIR")
        ));

        // Fallback threshold: if too many items to fit in a typical panel height,
        // assume scrollbar is visible before the user has scrolled
        let is_scrollable = self.is_scrollable || self.values.len() > 15;

        let mut list = column![].spacing(2);
        for (i, net) in self.values.iter().enumerate() {
            let hovered = self.hover_index == Some(i);
            list = list.push(range_row(i, net, trash_handle.clone(), hovered, is_scrollable));
        }

        scrollable(list)
            .on_scroll(|vp| {
                AddressListMessage::ScrollChanged(
                    vp.content_bounds().height > vp.bounds().height,
                )
            })
            .style(scrollable_style)
            .width(Fill)
            .height(Fill)
            .into()
    }
}

fn range_row(
    index: usize,
    net: &IpNet,
    trash_handle: svg::Handle,
    is_hovered: bool,
    is_scrollable: bool,
) -> Element<'_, AddressListMessage> {
    let row_content = row![
        text(net.to_string())
            .size(13)
            .font(MONO)
            .style(|t: &Theme| iced::widget::text::Style {
                color: Some(if is_dark(t) { c("#C4CAD4") } else { c("#3A4049") }),
            }),
        Space::new().width(Fill),
        text(format_host_count(net_host_count(net)))
            .size(11)
            .font(MONO)
            .style(|t: &Theme| iced::widget::text::Style {
                color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
            }),
        Space::new().width(10),
        button(
            svg(trash_handle)
                .content_fit(iced::ContentFit::Fill)
                .width(Fixed(14.0))
                .height(Fixed(14.0))
                .style(|t: &Theme, _| iced::widget::svg::Style {
                    color: Some(if is_dark(t) { c("#6B7480") } else { c("#8A929E") }),
                }),
        )
        .style(trash_btn_style)
        .padding(Padding::from([4, 4]))
        .on_press(AddressListMessage::RemoveClicked(index)),
    ]
    .align_y(Alignment::Center);

    let right_pad = if is_scrollable { 14.0 } else { 4.0 };

    let content = container(row_content)
        .style(move |t: &Theme| ContainerStyle {
            background: if is_hovered {
                Some(Background::Color(if is_dark(t) { c("#181D25") } else { c("#F2F4F7") }))
            } else {
                None
            },
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 7.0.into() },
            ..Default::default()
        })
        .padding(Padding { top: 8.0, right: right_pad, bottom: 8.0, left: 10.0 })
        .width(Fill);

    mouse_area(content)
        .on_enter(AddressListMessage::HoverEnter(index))
        .on_exit(AddressListMessage::HoverExit)
        .into()
}

fn net_host_count(net: &IpNet) -> u64 {
    match net {
        IpNet::V4(n) => {
            let bits = 32u64.saturating_sub(n.prefix_len() as u64);
            1u64 << bits
        }
        IpNet::V6(n) => {
            let bits = 128u64.saturating_sub(n.prefix_len() as u64);
            if bits >= 64 { u64::MAX } else { 1u64 << bits }
        }
    }
}

fn format_host_count(n: u64) -> String {
    if n == u64::MAX { "∞".to_string() } else { n.to_string() }
}

fn trash_btn_style(t: &Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
    use iced::widget::button::{Status, Style};
    use iced::{Border, Shadow};
    let red  = if is_dark(t) { c("#E5604D") } else { c("#CC3A28") };
    let idle = if is_dark(t) { c("#6B7480") } else { c("#8A929E") };
    let border = Border { color: Color::TRANSPARENT, width: 0.0, radius: 6.0.into() };
    match status {
        Status::Hovered => Style {
            background: Some(Background::Color(Color { r: 0.898, g: 0.376, b: 0.302, a: 0.15 })),
            text_color: red, border, shadow: Shadow::default(), snap: false,
        },
        Status::Pressed => Style {
            background: Some(Background::Color(Color { r: 0.898, g: 0.376, b: 0.302, a: 0.25 })),
            text_color: red, border, shadow: Shadow::default(), snap: false,
        },
        _ => Style {
            background: None,
            text_color: idle, border, shadow: Shadow::default(), snap: false,
        },
    }
}
