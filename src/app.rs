use std::sync::Arc;
use futures::channel::mpsc;
use futures::stream::BoxStream;
use futures::StreamExt;
use iced::{window, Element, Subscription, Task, Theme};
use crate::components::{address_list, settings};
use crate::components::address_list::{AddressList, AddressListMessage};
use crate::components::results_list::{ResultsList, ResultsListMessage};
use crate::i18n::{self, Language, Tr};
use crate::scanner::parse::{parse_ip_ranges, parse_ports};
use crate::scanner::types::{ScanConfig, ServerInfo};
use crate::styles::{COLOR_THEME, COLOR_THEME_LIGHT};

#[derive(Debug, Clone, PartialEq)]
pub enum ModalKind {
    None,
    Settings,
    AddRanges,
}

#[derive(Debug, Clone)]
pub enum Message {
    WindowInitialized(Option<window::Id>),
    ScanStart,
    ScanStop,
    ServerFound(ServerInfo),
    ScanProgress(usize),
    ScanComplete,
    AddressList(AddressListMessage),
    ResultsList(ResultsListMessage),
    JavaPortsChanged(String),
    BedrockPortsChanged(String),
    ConcurrencyChanged(String),
    TimeoutChanged(String),
    OpenModal(ModalKind),
    CloseModal,
    RangesEditorAction(iced::widget::text_editor::Action),
    ConfirmAddRanges,
    SetTheme(bool),
    SetLanguage(Language),
}

pub struct ScanSettings {
    pub(crate) java_ports: String,
    pub(crate) bedrock_ports: String,
    pub(crate) concurrency: String,
    pub(crate) timeout_ms: String,
    pub(crate) java_ports_error: bool,
    pub(crate) bedrock_ports_error: bool,
}

impl Default for ScanSettings {
    fn default() -> Self {
        Self {
            java_ports: "25565".into(),
            bedrock_ports: "19132".into(),
            concurrency: "1024".into(),
            timeout_ms: "1500".into(),
            java_ports_error: false,
            bedrock_ports_error: false,
        }
    }
}

impl ScanSettings {
    fn java_ports_parsed(&self) -> Vec<u16> { parse_ports(&self.java_ports) }
    fn bedrock_ports_parsed(&self) -> Vec<u16> { parse_ports(&self.bedrock_ports) }
}

pub struct McScan {
    pub(crate) wid: Option<window::Id>,
    pub(crate) results: ResultsList,
    pub(crate) address_list: AddressList,
    pub(crate) settings: ScanSettings,
    pub(crate) is_scanning: bool,
    pub(crate) scan_id: u64,
    pub(crate) total_targets: usize,
    pub(crate) scanned_count: usize,
    pub(crate) modal: ModalKind,
    pub(crate) ranges_editor: iced::widget::text_editor::Content,
    pub(crate) is_dark: bool,
    pub(crate) language: Language,
}

impl McScan {
    pub fn init() -> (Self, Task<Message>) {
        let app = Self {
            wid: None,
            results: ResultsList::default(),
            address_list: AddressList::default(),
            settings: ScanSettings::default(),
            is_scanning: false,
            scan_id: 0,
            total_targets: 0,
            scanned_count: 0,
            modal: ModalKind::None,
            ranges_editor: iced::widget::text_editor::Content::new(),
            is_dark: true,
            language: Language::detect(),
        };
        (app, Task::discard(window::latest()).map(Message::WindowInitialized))
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::WindowInitialized(id) => self.wid = id,

            Message::ScanStart => {
                let jp = self.settings.java_ports_parsed();
                let bp = self.settings.bedrock_ports_parsed();
                self.settings.java_ports_error = jp.is_empty();
                self.settings.bedrock_ports_error = bp.is_empty();

                if self.address_list.values().is_empty() {
                    self.ranges_editor = iced::widget::text_editor::Content::new();
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
                self.results.push(info);
            }

            Message::ScanProgress(n) => {
                if self.is_scanning {
                    self.scanned_count = n;
                }
            }

            Message::ScanComplete => {
                self.scanned_count = self.total_targets;
                self.is_scanning = false;
            }

            Message::AddressList(msg) => self.address_list.update(msg),
            Message::ResultsList(_) => {}

            Message::JavaPortsChanged(v) => {
                self.settings.java_ports_error = false;
                self.settings.java_ports = v;
            }
            Message::BedrockPortsChanged(v) => {
                self.settings.bedrock_ports_error = false;
                self.settings.bedrock_ports = v;
            }
            Message::ConcurrencyChanged(v) => self.settings.concurrency = v,
            Message::TimeoutChanged(v) => self.settings.timeout_ms = v,

            Message::OpenModal(kind) => self.modal = kind,
            Message::CloseModal => self.modal = ModalKind::None,

            Message::RangesEditorAction(action) => self.ranges_editor.perform(action),

            Message::ConfirmAddRanges => {
                let raw = self.ranges_editor.text();
                let ranges = parse_ip_ranges(&raw);
                self.address_list.push_ranges(ranges);
                self.ranges_editor = iced::widget::text_editor::Content::new();
                self.modal = ModalKind::None;
            }

            Message::SetTheme(dark) => self.is_dark = dark,
            Message::SetLanguage(lang) => self.language = lang,
        }
    }

    pub fn tr(&self) -> &'static Tr {
        i18n::tr(self.language)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if !self.is_scanning {
            return Subscription::none();
        }
        let config = Arc::new(self.scan_config());
        Subscription::run_with((self.scan_id, config), build_scan_stream)
    }

    pub fn theme(&self) -> Theme {
        if self.is_dark { COLOR_THEME.clone() } else { COLOR_THEME_LIGHT.clone() }
    }

    pub fn view(&self) -> Element<'_, Message> {
        use iced::widget::{container, row, Space, Stack};
        use iced::{Fill, Length::Fixed};
        use crate::components::{left_panel, right_panel};
        use crate::styles::{c, is_dark};

        let sep = container(Space::new())
            .width(Fixed(1.0))
            .height(Fill)
            .style(|t: &iced::Theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(if is_dark(t) { c("#1A1F27") } else { c("#E1E5EA") })),
                ..Default::default()
            });

        let base = container(
            row![
                container(left_panel::render(self)).width(Fill).height(Fill),
                sep,
                container(right_panel::render(self)).width(Fixed(340.0)).height(Fill),
            ]
            .width(Fill)
            .height(Fill),
        )
        .style(app_bg_style)
        .width(Fill)
        .height(Fill)
        .into();

        match &self.modal {
            ModalKind::None      => base,
            ModalKind::Settings  => Stack::new().push(base).push(settings::render(self)).into(),
            ModalKind::AddRanges => Stack::new().push(base).push(address_list::add_dialog::render(self)).into(),
        }
    }

    fn scan_config(&self) -> ScanConfig {
        ScanConfig {
            ranges: self.address_list.values().to_vec(),
            java_ports: self.settings.java_ports_parsed(),
            bedrock_ports: self.settings.bedrock_ports_parsed(),
            concurrency: self.settings.concurrency.parse().unwrap_or(1024).max(1),
            timeout_ms: self.settings.timeout_ms.parse().unwrap_or(1500).max(100),
        }
    }
}

fn app_bg_style(t: &iced::Theme) -> iced::widget::container::Style {
    use crate::styles::{c, is_dark};
    iced::widget::container::Style {
        background: Some(iced::Background::Color(if is_dark(t) { c("#0E1116") } else { c("#F0F1F3") })),
        ..Default::default()
    }
}

fn build_scan_stream(data: &(u64, Arc<ScanConfig>)) -> BoxStream<'static, Message> {
    let config = data.1.clone();
    let (tx, rx) = mpsc::unbounded();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");

        rt.block_on(async move {
            let mut stream = Box::pin(crate::scanner::scan(config));
            let mut scanned = 0usize;
            while let Some(maybe_info) = stream.next().await {
                scanned += 1;
                if let Some(info) = maybe_info {
                    if tx.unbounded_send(Message::ServerFound(info)).is_err() {
                        return;
                    }
                }
                if scanned % 512 == 0 {
                    if tx.unbounded_send(Message::ScanProgress(scanned)).is_err() {
                        return;
                    }
                }
            }
            let _ = tx.unbounded_send(Message::ScanComplete);
        });
    });

    Box::pin(rx)
}
