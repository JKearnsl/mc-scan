use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{button, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Background, Border, Color, Element, Fill, Padding};
use iced::Length::Fixed;
use ipnet::IpNet;
use crate::styles::{c, scrollable_style, trash_button_style, MONO};

#[derive(Debug, Clone)]
pub enum AddressListMessage {
    RemoveClicked(usize),
    HoverEnter(usize),
    HoverExit,
}

pub struct AddressList {
    values: Vec<IpNet>,
    hover_index: Option<usize>,
}

impl Default for AddressList {
    fn default() -> Self {
        Self { values: Vec::new(), hover_index: None }
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
            AddressListMessage::HoverExit      => self.hover_index = None,
        }
    }

    pub fn total_hosts(&self) -> u64 {
        self.values.iter().map(net_host_count).sum()
    }

    pub fn view(&self, dark: bool) -> Element<'_, AddressListMessage> {
        let trash_handle = svg::Handle::from_path(format!(
            "{}/assets/trash.svg",
            env!("CARGO_MANIFEST_DIR")
        ));

        let mut list = column![].spacing(2);
        for (i, net) in self.values.iter().enumerate() {
            let hovered = self.hover_index == Some(i);
            list = list.push(range_row(i, net, trash_handle.clone(), hovered, dark));
        }

        scrollable(list)
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
    dark: bool,
) -> Element<'_, AddressListMessage> {
    let cidr_color  = if dark { c("#C4CAD4") } else { c("#3A4049") };
    let count_color = if dark { c("#5C636F") } else { c("#A0A7B1") };
    let svg_color   = if dark { c("#6B7480") } else { c("#8A929E") };
    let hover_bg: Option<Color> = if is_hovered {
        Some(if dark { c("#181D25") } else { c("#F2F4F7") })
    } else {
        None
    };

    let row_content = row![
        text(net.to_string())
            .size(13)
            .font(MONO)
            .style(move |_| iced::widget::text::Style { color: Some(cidr_color) }),
        Space::new().width(Fill),
        text(format_host_count(net_host_count(net)))
            .size(11)
            .font(MONO)
            .style(move |_| iced::widget::text::Style { color: Some(count_color) }),
        Space::new().width(10),
        button(
            svg(trash_handle)
                .content_fit(iced::ContentFit::Fill)
                .width(Fixed(14.0))
                .height(Fixed(14.0))
                .style(move |_theme, _status| iced::widget::svg::Style { color: Some(svg_color) }),
        )
        .style(trash_button_style)
        .padding(Padding::from([4, 4]))
        .on_press(AddressListMessage::RemoveClicked(index)),
    ]
    .align_y(Alignment::Center);

    let content = container(row_content)
        .style(move |_: &_| ContainerStyle {
            background: hover_bg.map(Background::Color),
            border: Border { color: Color::TRANSPARENT, width: 0.0, radius: 7.0.into() },
            ..Default::default()
        })
        .padding(Padding { top: 8.0, right: 0.0, bottom: 8.0, left: 10.0 })
        .width(Fill);

    mouse_area(content)
        .on_enter(AddressListMessage::HoverEnter(index))
        .on_exit(AddressListMessage::HoverExit)
        .into()
}

fn net_host_count(net: &IpNet) -> u64 {
    match net {
        IpNet::V4(n) => { let bits = 32u64.saturating_sub(n.prefix_len() as u64); 1u64 << bits }
        IpNet::V6(n) => { let bits = 128u64.saturating_sub(n.prefix_len() as u64); if bits >= 64 { u64::MAX } else { 1u64 << bits } }
    }
}

fn format_host_count(n: u64) -> String {
    if n == u64::MAX { "∞".to_string() } else { n.to_string() }
}
