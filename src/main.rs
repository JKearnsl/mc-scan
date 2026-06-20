mod components;
mod scanner;
mod styles;

use components::address_list::{AddressList, AddressListMessage};
use components::results_list::{ResultsList, ResultsListMessage};
use scanner::types::{ScanConfig, ServerInfo};
use styles::{
    add_button_style, app_bg_style, button_danger, button_primary, button_secondary, c,
    header_style, icon_button_style, progress_bar_style, right_panel_style, text_editor_style,
    text_input_error_style, text_input_style, COLOR_THEME, COLOR_THEME_LIGHT, MONO, MONO_SEMIBOLD,
    SANS, SANS_SEMIBOLD,
};

use futures::channel::mpsc;
use futures::stream::BoxStream;
use futures::StreamExt;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::space::Space;
use iced::widget::{
    button, column, container, progress_bar, row, svg, text, text_editor, text_input, Stack,
};
use iced::Length::Fixed;
use iced::{
    window, Alignment, Background, Border, Color, ContentFit, Element, Fill, Font, Padding, Size,
    Subscription, Task, Theme,
};
use ipnet::IpNet;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

const PLEX_SANS: &[u8] = include_bytes!("../fonts/IBMPlexSans-Regular.ttf");
const PLEX_SANS_MEDIUM: &[u8] = include_bytes!("../fonts/IBMPlexSans-Medium.ttf");
const PLEX_SANS_SEMIBOLD: &[u8] = include_bytes!("../fonts/IBMPlexSans-SemiBold.ttf");
const PLEX_MONO: &[u8] = include_bytes!("../fonts/IBMPlexMono-Medium.ttf");
const PLEX_MONO_SEMIBOLD: &[u8] = include_bytes!("../fonts/IBMPlexMono-SemiBold.ttf");

#[derive(Debug, Clone, PartialEq)]
enum ModalKind {
    None,
    Settings,
    AddRanges,
}

struct McScan {
    wid: Option<window::Id>,
    results: ResultsList,
    address_list: AddressList,
    java_ports_input: String,
    bedrock_ports_input: String,
    concurrency_input: String,
    timeout_input: String,
    java_ports_error: bool,
    bedrock_ports_error: bool,
    is_scanning: bool,
    scan_id: u64,
    total_targets: usize,
    scanned_count: usize,
    modal: ModalKind,
    ranges_content: text_editor::Content,
    is_dark: bool,
}

#[derive(Debug, Clone)]
enum Message {
    WindowInitialized(Option<window::Id>),
    ScanStart,
    ScanStop,
    ServerFound(ServerInfo),
    ScanComplete,
    AddressList(AddressListMessage),
    ResultsList(ResultsListMessage),
    JavaPortsChanged(String),
    BedrockPortsChanged(String),
    ConcurrencyChanged(String),
    TimeoutChanged(String),
    OpenModal(ModalKind),
    CloseModal,
    RangesEditorAction(text_editor::Action),
    ConfirmAddRanges,
    SetTheme(bool),
}

impl McScan {
    fn init() -> (Self, Task<Message>) {
        let app = Self {
            wid: None,
            results: ResultsList::default(),
            address_list: AddressList::default(),
            java_ports_input: "25565".into(),
            bedrock_ports_input: "19132".into(),
            concurrency_input: "1024".into(),
            timeout_input: "1500".into(),
            java_ports_error: false,
            bedrock_ports_error: false,
            is_scanning: false,
            scan_id: 0,
            total_targets: 0,
            scanned_count: 0,
            modal: ModalKind::None,
            ranges_content: text_editor::Content::new(),
            is_dark: true,
        };
        (
            app,
            Task::discard(window::latest()).map(Message::WindowInitialized),
        )
    }

    fn scan_config(&self) -> ScanConfig {
        ScanConfig {
            ranges: self.address_list.values().to_vec(),
            java_ports: parse_ports(&self.java_ports_input),
            bedrock_ports: parse_ports(&self.bedrock_ports_input),
            concurrency: self.concurrency_input.parse().unwrap_or(1024).max(1),
            timeout_ms: self.timeout_input.parse().unwrap_or(1500).max(100),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::WindowInitialized(id) => self.wid = id,

            Message::ScanStart => {
                let jp = parse_ports(&self.java_ports_input);
                let bp = parse_ports(&self.bedrock_ports_input);
                self.java_ports_error = jp.is_empty();
                self.bedrock_ports_error = bp.is_empty();

                if self.address_list.values().is_empty() {
                    self.ranges_content = text_editor::Content::new();
                    self.modal = ModalKind::AddRanges;
                    return;
                }
                if jp.is_empty() && bp.is_empty() {
                    self.modal = ModalKind::Settings;
                    return;
                }

                self.results.clear();
                let config = self.scan_config();
                self.total_targets = config.target_count();
                self.scanned_count = 0;
                self.scan_id += 1;
                self.is_scanning = true;
            }

            Message::ScanStop => self.is_scanning = false,

            Message::ServerFound(info) => {
                self.scanned_count += 1;
                self.results.push(info);
            }

            Message::ScanComplete => {
                self.scanned_count = self.total_targets;
                self.is_scanning = false;
            }

            Message::AddressList(msg) => self.address_list.update(msg),
            Message::ResultsList(_) => {}

            Message::JavaPortsChanged(v) => {
                self.java_ports_error = false;
                self.java_ports_input = v;
            }
            Message::BedrockPortsChanged(v) => {
                self.bedrock_ports_error = false;
                self.bedrock_ports_input = v;
            }
            Message::ConcurrencyChanged(v) => self.concurrency_input = v,
            Message::TimeoutChanged(v) => self.timeout_input = v,

            Message::OpenModal(kind) => self.modal = kind,
            Message::CloseModal => self.modal = ModalKind::None,

            Message::RangesEditorAction(action) => self.ranges_content.perform(action),

            Message::ConfirmAddRanges => {
                let raw = self.ranges_content.text();
                let ranges = parse_ip_ranges(&raw);
                self.address_list.push_ranges(ranges);
                self.ranges_content = text_editor::Content::new();
                self.modal = ModalKind::None;
            }

            Message::SetTheme(dark) => self.is_dark = dark,
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if !self.is_scanning {
            return Subscription::none();
        }
        let config = Arc::new(self.scan_config());
        Subscription::run_with((self.scan_id, config), build_scan_stream)
    }

    fn view(&self) -> Element<'_, Message> {
        let base = self.base_view();
        match &self.modal {
            ModalKind::None => base,
            ModalKind::Settings => Stack::new()
                .push(base)
                .push(self.settings_modal())
                .into(),
            ModalKind::AddRanges => Stack::new()
                .push(base)
                .push(self.add_ranges_modal())
                .into(),
        }
    }

    fn base_view(&self) -> Element<'_, Message> {
        container(
            row![
                container(self.left_panel()).width(Fill).height(Fill),
                container(self.right_panel())
                    .width(Fixed(340.0))
                    .height(Fill),
            ]
            .width(Fill)
            .height(Fill),
        )
        .style(app_bg_style)
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn left_panel(&self) -> Element<'_, Message> {
        let dark = self.is_dark;
        let found_count = self.results.count();

        let badge_bg_color = if dark {
            Color { r: 0.239, g: 0.839, b: 0.549, a: 0.12 }
        } else {
            Color { r: 0.094, g: 0.659, b: 0.384, a: 0.08 }
        };
        let badge_brd_color = if dark {
            Color { r: 0.239, g: 0.839, b: 0.549, a: 0.25 }
        } else {
            Color { r: 0.094, g: 0.659, b: 0.384, a: 0.20 }
        };
        let badge_dot_c  = if dark { c("#3DD68C") } else { c("#18A862") };
        let badge_text_c = if dark { c("#9FE9C4") } else { c("#0B6040") };
        let title_c      = if dark { c("#E8EBF0") } else { c("#161A20") };
        let subtitle_c   = if dark { c("#6B7480") } else { c("#8A929E") };
        let divider_c    = if dark { c("#1A1F27") } else { c("#E1E5EA") };

        // ── badge: зелёная точка + "N найдено" ──
        let badge = container(
            row![
                text("●")
                    .size(9)
                    .font(MONO)
                    .style(move |_| iced::widget::text::Style { color: Some(badge_dot_c) }),
                text(format!("{} найдено", found_count))
                    .size(13)
                    .font(MONO)
                    .style(move |_| iced::widget::text::Style { color: Some(badge_text_c) }),
            ]
            .align_y(Alignment::Center)
            .spacing(8),
        )
        .style(move |_: &_| ContainerStyle {
            background: Some(Background::Color(badge_bg_color)),
            border: Border {
                color: badge_brd_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .padding(Padding { top: 5.0, right: 11.0, bottom: 5.0, left: 11.0 });

        // ── строка заголовка ──
        let title_group = row![
            text("mc-scan")
                .size(18)
                .font(MONO_SEMIBOLD)
                .style(move |_| iced::widget::text::Style { color: Some(title_c) }),
            text("сканер Minecraft-серверов")
                .size(13)
                .font(SANS)
                .style(move |_| iced::widget::text::Style { color: Some(subtitle_c) }),
        ]
        .align_y(Alignment::Center)
        .spacing(12);

        let title_row = row![title_group, Space::new().width(Fill), badge]
            .align_y(Alignment::Center);

        let mut header_col = column![title_row].spacing(0);

        if self.is_scanning && self.total_targets > 0 {
            let ratio = self.scanned_count as f32 / self.total_targets as f32;
            let pct = (ratio * 100.0) as u32;
            let range_str = self
                .address_list
                .values()
                .first()
                .map(|r| r.to_string())
                .unwrap_or_else(|| "…".to_string());
            let scan_range_c = subtitle_c;
            let scan_stats_c = if dark { c("#8C95A3") } else { c("#6B7480") };

            header_col = header_col
                .push(Space::new().height(Fixed(14.0)))
                .push(
                    progress_bar(0.0..=1.0, ratio)
                        .style(progress_bar_style)
                        .girth(Fixed(4.0))
                        .length(Fill),
                )
                .push(Space::new().height(Fixed(9.0)))
                .push(
                    row![
                        text(format!("Сканирование {}", range_str))
                            .size(12)
                            .font(MONO)
                            .style(move |_| iced::widget::text::Style { color: Some(scan_range_c) }),
                        Space::new().width(Fill),
                        text(format!(
                            "{}% · {} / {}",
                            pct, self.scanned_count, self.total_targets
                        ))
                        .size(12)
                        .font(MONO)
                        .style(move |_| iced::widget::text::Style { color: Some(scan_stats_c) }),
                    ]
                    .align_y(Alignment::Center),
                );
        }

        let header = container(header_col)
            .style(header_style)
            .padding(Padding { top: 18.0, right: 20.0, bottom: 16.0, left: 20.0 })
            .width(Fill);

        let divider = container(Space::new().height(Fixed(0.0)))
            .style(move |_: &_| ContainerStyle {
                background: Some(Background::Color(divider_c)),
                ..Default::default()
            })
            .width(Fill)
            .height(Fixed(1.0));

        let results = self.results.view(dark).map(Message::ResultsList);

        let col = column![header, divider, results].width(Fill).height(Fill);
        container(col).width(Fill).height(Fill).into()
    }

    fn right_panel(&self) -> Element<'_, Message> {
        let dark = self.is_dark;

        let svg_c         = if dark { c("#6B7480") } else { c("#5B6470") };
        let label_c       = if dark { c("#5C636F") } else { c("#A0A7B1") };
        let total_val_c   = if dark { c("#A2ABBA") } else { c("#3A4049") };
        let divider_c     = if dark { c("#1A1F27") } else { c("#E1E5EA") };

        let settings_icon = svg::Handle::from_path(format!(
            "{}/assets/settings.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let settings_btn = button(
            svg(settings_icon)
                .content_fit(ContentFit::Fill)
                .width(Fixed(20.0))
                .height(Fixed(20.0))
                .style(move |_theme, _status| svg::Style { color: Some(svg_c) }),
        )
        .style(icon_button_style)
        .padding(Padding::from([13, 13]))
        .width(Fixed(48.0))
        .height(Fixed(48.0))
        .on_press(Message::OpenModal(ModalKind::Settings));

        let action_btn = if self.is_scanning {
            button(
                container(
                    text("■  Стоп")
                        .size(16)
                        .font(SANS_SEMIBOLD)
                        .style(|_| iced::widget::text::Style { color: Some(c("#FFFFFF")) }),
                )
                .center(Fill),
            )
            .style(button_danger)
            .on_press(Message::ScanStop)
            .width(Fill)
            .height(Fixed(48.0))
        } else {
            let can_scan = !self.address_list.values().is_empty();
            let scan_text_color = if can_scan {
                if dark { c("#08110B") } else { c("#FFFFFF") }
            } else {
                if dark { c("#5C636F") } else { c("#A0A7B1") }
            };
            let btn = button(
                container(
                    text("▶  Сканировать")
                        .size(16)
                        .font(SANS_SEMIBOLD)
                        .style(move |_| iced::widget::text::Style { color: Some(scan_text_color) }),
                )
                .center(Fill),
            )
            .style(button_primary)
            .width(Fill)
            .height(Fixed(48.0));
            if can_scan { btn.on_press(Message::ScanStart) } else { btn }
        };

        let top_row = row![action_btn, Space::new().width(8), settings_btn]
            .align_y(Alignment::Center);

        let range_count = self.address_list.values().len();
        let add_btn = button("+")
            .on_press(Message::OpenModal(ModalKind::AddRanges))
            .padding(Padding::from([3, 8]))
            .style(add_button_style)
            .width(Fixed(26.0))
            .height(Fixed(26.0));

        let ranges_header = row![
            text(format!("IP-ДИАПАЗОНЫ · {}", range_count))
                .size(11)
                .font(SANS_SEMIBOLD)
                .style(move |_| iced::widget::text::Style { color: Some(label_c) }),
            Space::new().width(Fill),
            add_btn,
        ]
        .align_y(Alignment::Center);

        let ranges_list = self.address_list.view(dark).map(Message::AddressList);

        let total_hosts = self.address_list.total_hosts();
        let total_str = if total_hosts == u64::MAX { "∞".to_string() } else { total_hosts.to_string() };

        let total_row = row![
            text("Всего адресов")
                .size(12)
                .font(SANS)
                .style(move |_| iced::widget::text::Style { color: Some(label_c) }),
            Space::new().width(Fill),
            text(total_str)
                .size(12)
                .font(MONO)
                .style(move |_| iced::widget::text::Style { color: Some(total_val_c) }),
        ]
        .align_y(Alignment::Center);

        let divider = container(Space::new().height(1))
            .style(move |_: &_| ContainerStyle {
                background: Some(Background::Color(divider_c)),
                ..Default::default()
            })
            .width(Fill)
            .height(Fixed(1.0));

        container(
            column![
                top_row,
                Space::new().height(16),
                ranges_header,
                Space::new().height(6),
                ranges_list,
                Space::new().height(8),
                divider,
                Space::new().height(8),
                total_row,
            ]
            .padding(Padding::from([16, 16]))
            .width(Fill)
            .height(Fill),
        )
        .style(right_panel_style)
        .width(Fixed(340.0))
        .height(Fill)
        .into()
    }

    fn settings_modal(&self) -> Element<'_, Message> {
        let dark = self.is_dark;

        let title_c    = if dark { c("#E8EBF0") } else { c("#161A20") };
        let dialog_bg  = if dark { c("#0E1116") } else { c("#FFFFFF") };
        let dialog_brd = if dark { c("#232A34") } else { c("#E1E5EA") };
        let done_text_c = if dark { c("#08110B") } else { c("#FFFFFF") };

        let dialog = container(
            column![
                row![
                    text("Настройки")
                        .size(16)
                        .font(SANS_SEMIBOLD)
                        .style(move |_| iced::widget::text::Style { color: Some(title_c) }),
                    Space::new().width(Fill),
                    button("×")
                        .on_press(Message::CloseModal)
                        .style(icon_button_style)
                        .padding(Padding::from([4, 10])),
                ]
                .align_y(Alignment::Center),
                Space::new().height(16),
                section_label("ТЕМА", dark),
                Space::new().height(8),
                row![
                    button(
                        container(text("Тёмная").size(13).font(SANS_SEMIBOLD))
                            .center(Fill),
                    )
                    .style(if self.is_dark { button_primary } else { button_secondary })
                    .on_press(Message::SetTheme(true))
                    .width(Fill)
                    .height(Fixed(36.0)),
                    Space::new().width(8),
                    button(
                        container(text("Светлая").size(13).font(SANS_SEMIBOLD))
                            .center(Fill),
                    )
                    .style(if !self.is_dark { button_primary } else { button_secondary })
                    .on_press(Message::SetTheme(false))
                    .width(Fill)
                    .height(Fixed(36.0)),
                ],
                Space::new().height(16),
                section_label("ПОРТЫ", dark),
                Space::new().height(8),
                labeled_input(
                    "Java",
                    &self.java_ports_input,
                    "25565",
                    Message::JavaPortsChanged,
                    self.java_ports_error,
                    dark,
                ),
                Space::new().height(6),
                labeled_input(
                    "Bedrock",
                    &self.bedrock_ports_input,
                    "19132",
                    Message::BedrockPortsChanged,
                    self.bedrock_ports_error,
                    dark,
                ),
                Space::new().height(16),
                section_label("ПАРАМЕТРЫ", dark),
                Space::new().height(8),
                labeled_input(
                    "Потоки",
                    &self.concurrency_input,
                    "1024",
                    Message::ConcurrencyChanged,
                    false,
                    dark,
                ),
                Space::new().height(6),
                labeled_input(
                    "Таймаут мс",
                    &self.timeout_input,
                    "1500",
                    Message::TimeoutChanged,
                    false,
                    dark,
                ),
                Space::new().height(20),
                button(
                    container(
                        text("Готово")
                            .size(15)
                            .font(SANS_SEMIBOLD)
                            .style(move |_| iced::widget::text::Style { color: Some(done_text_c) }),
                    )
                    .center(Fill),
                )
                .on_press(Message::CloseModal)
                .style(button_primary)
                .width(Fill)
                .height(Fixed(44.0)),
            ]
            .padding(Padding::from([20, 24])),
        )
        .width(Fixed(380.0))
        .style(move |_: &_| ContainerStyle {
            background: Some(Background::Color(dialog_bg)),
            border: Border { color: dialog_brd, width: 1.0, radius: 12.0.into() },
            ..Default::default()
        });

        modal_backdrop(dialog.into())
    }

    fn add_ranges_modal(&self) -> Element<'_, Message> {
        let dark = self.is_dark;

        let title_c    = if dark { c("#E8EBF0") } else { c("#161A20") };
        let hint_c     = if dark { c("#5C636F") } else { c("#A0A7B1") };
        let dialog_bg  = if dark { c("#0E1116") } else { c("#FFFFFF") };
        let dialog_brd = if dark { c("#232A34") } else { c("#E1E5EA") };
        let add_text_c = if dark { c("#08110B") } else { c("#FFFFFF") };

        let dialog = container(
            column![
                row![
                    text("Добавить диапазоны")
                        .size(16)
                        .font(SANS_SEMIBOLD)
                        .style(move |_| iced::widget::text::Style { color: Some(title_c) }),
                    Space::new().width(Fill),
                    button("×")
                        .on_press(Message::CloseModal)
                        .style(icon_button_style)
                        .padding(Padding::from([4, 10])),
                ]
                .align_y(Alignment::Center),
                Space::new().height(4),
                text("CIDR (10.0.0.0/8) · диапазон (1.2.3.4-1.2.3.100) · одиночный IP")
                    .size(11)
                    .font(SANS)
                    .style(move |_| iced::widget::text::Style { color: Some(hint_c) }),
                Space::new().height(12),
                text_editor(&self.ranges_content)
                    .on_action(Message::RangesEditorAction)
                    .height(Fixed(160.0))
                    .style(text_editor_style)
                    .font(MONO)
                    .size(13)
                    .padding(Padding::from([8, 10])),
                Space::new().height(16),
                row![
                    button(
                        container(
                            text("Отмена")
                                .size(14)
                                .font(SANS_SEMIBOLD)
                                .style(|_| iced::widget::text::Style { color: Some(c("#FFFFFF")) }),
                        )
                        .center(Fill),
                    )
                    .on_press(Message::CloseModal)
                    .style(button_danger)
                    .width(Fill)
                    .height(Fixed(44.0)),
                    Space::new().width(10),
                    button(
                        container(
                            text("Добавить")
                                .size(14)
                                .font(SANS_SEMIBOLD)
                                .style(move |_| iced::widget::text::Style { color: Some(add_text_c) }),
                        )
                        .center(Fill),
                    )
                    .on_press(Message::ConfirmAddRanges)
                    .style(button_primary)
                    .width(Fill)
                    .height(Fixed(44.0)),
                ],
            ]
            .padding(Padding::from([20, 24])),
        )
        .width(Fixed(460.0))
        .style(move |_: &_| ContainerStyle {
            background: Some(Background::Color(dialog_bg)),
            border: Border { color: dialog_brd, width: 1.0, radius: 12.0.into() },
            ..Default::default()
        });

        modal_backdrop(dialog.into())
    }

    fn theme(&self) -> Theme {
        if self.is_dark {
            COLOR_THEME.clone()
        } else {
            COLOR_THEME_LIGHT.clone()
        }
    }
}

fn modal_backdrop(content: Element<'_, Message>) -> Element<'_, Message> {
    container(content)
        .center_x(Fill)
        .center_y(Fill)
        .style(|_| ContainerStyle {
            background: Some(Background::Color(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.70 })),
            ..Default::default()
        })
        .into()
}

fn section_label<'a>(label: &'a str, dark: bool) -> Element<'a, Message> {
    let color = if dark { c("#5C636F") } else { c("#A0A7B1") };
    text(label)
        .size(11)
        .font(SANS_SEMIBOLD)
        .style(move |_| iced::widget::text::Style { color: Some(color) })
        .into()
}

fn labeled_input<'a>(
    label: &'a str,
    value: &'a str,
    placeholder: &'a str,
    on_change: fn(String) -> Message,
    error: bool,
    dark: bool,
) -> Element<'a, Message> {
    let label_color = if dark { c("#A2ABBA") } else { c("#3A4049") };
    let style: fn(&Theme, _) -> _ = if error { text_input_error_style } else { text_input_style };
    row![
        text(label)
            .size(13)
            .font(SANS)
            .width(Fixed(90.0))
            .style(move |_| iced::widget::text::Style { color: Some(label_color) }),
        text_input(placeholder, value)
            .on_input(on_change)
            .padding(Padding::from([7, 10]))
            .size(13)
            .font(MONO)
            .style(style)
            .width(Fill),
    ]
    .align_y(Alignment::Center)
    .spacing(8)
    .into()
}

fn parse_ports(input: &str) -> Vec<u16> {
    input
        .split(',')
        .filter_map(|s| s.trim().parse::<u16>().ok())
        .collect()
}

fn parse_ip_ranges(input: &str) -> Vec<IpNet> {
    let mut result = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Ok(net) = line.parse::<IpNet>() {
            result.push(net);
            continue;
        }

        if let Ok(ip) = line.parse::<IpAddr>() {
            let prefix = if ip.is_ipv4() { 32 } else { 128 };
            if let Ok(net) = IpNet::new(ip, prefix) {
                result.push(net);
            }
            continue;
        }

        if let Some((start_str, end_str)) = line.split_once('-') {
            if let (Ok(start), Ok(end)) = (
                start_str.trim().parse::<Ipv4Addr>(),
                end_str.trim().parse::<Ipv4Addr>(),
            ) {
                result.extend(range_to_cidrs(start, end));
            }
        }
    }
    result
}

fn range_to_cidrs(start: Ipv4Addr, end: Ipv4Addr) -> Vec<IpNet> {
    let mut result = Vec::new();
    let mut s = u32::from(start);
    let e = u32::from(end);
    if s > e {
        return result;
    }

    while s <= e {
        let trailing = if s == 0 { 32u32 } else { s.trailing_zeros() };
        let mut prefix = (32u32 - trailing.min(32)) as u8;

        loop {
            let block_size = 1u64 << (32 - prefix);
            let block_end = s as u64 + block_size - 1;
            if block_end <= e as u64 {
                break;
            }
            prefix += 1;
        }

        result.push(IpNet::new(IpAddr::V4(Ipv4Addr::from(s)), prefix).unwrap());

        let next = s as u64 + (1u64 << (32 - prefix));
        if next > u32::MAX as u64 {
            break;
        }
        s = next as u32;
    }

    result
}

fn build_scan_stream(data: &(u64, Arc<ScanConfig>)) -> BoxStream<'static, Message> {
    let config = data.1.clone();
    let (tx, rx) = mpsc::unbounded();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");

        rt.block_on(async move {
            let mut stream = Box::pin(scanner::scan(config));
            while let Some(info) = stream.next().await {
                if tx.unbounded_send(Message::ServerFound(info)).is_err() {
                    return;
                }
            }
            let _ = tx.unbounded_send(Message::ScanComplete);
        });
    });

    Box::pin(rx)
}

fn main() -> iced::Result {
    iced::application(McScan::init, McScan::update, McScan::view)
        .title("mc-scan")
        .theme(McScan::theme)
        .subscription(McScan::subscription)
        .font(PLEX_SANS)
        .font(PLEX_SANS_MEDIUM)
        .font(PLEX_SANS_SEMIBOLD)
        .font(PLEX_MONO)
        .font(PLEX_MONO_SEMIBOLD)
        .default_font(Font::with_name("IBM Plex Sans"))
        .window(window::Settings {
            size: Size { width: 1060.0, height: 620.0 },
            min_size: Some(Size { width: 780.0, height: 480.0 }),
            resizable: true,
            ..window::Settings::default()
        })
        .run()
}
