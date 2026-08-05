use crate::components::address_list::{AddressList, AddressListMessage};
use crate::components::results_list::{ResultsList, ResultsListMessage};
use crate::components::{address_list, settings};
use crate::i18n::{self, Language, Tr};
use crate::styles::{COLOR_THEME, COLOR_THEME_LIGHT};
use futures::StreamExt;
use futures::channel::{mpsc, oneshot};
use futures::stream::BoxStream;
use iced::{Element, Subscription, Task, Theme, window};
use once_cell::sync::Lazy;
use scanner::limits::{Concurrency, Ports, TimeoutMs};
use scanner::parse::{parse_ip_ranges, parse_ip_ranges_reporting};
use scanner::types::{ScanConfig, ServerInfo};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build shared tokio runtime")
});

const REFRESH_TIMER_ID: u8 = 0;
const THEME_SUB_ID: u8 = 1;

#[derive(Debug, Clone, PartialEq)]
pub enum ModalKind {
    None,
    Settings,
    AddRanges,
    ServerPreview(SocketAddr),
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
    ToggleQuery(bool),
    ToggleOnlineModeCheck(bool),
    OpenModal(ModalKind),
    CloseModal,
    RangesEditorAction(iced::widget::text_editor::Action),
    ConfirmAddRanges,
    SetThemePref(crate::config::ThemePref),
    SystemColorScheme(bool),
    SetLanguage(Language),
    CopyAddress,
    CopiedReset,
    ToggleVersionExpand,
    ExportResults,
    RefreshTick,
    ServerRefreshed(Option<ServerInfo>),
    AvatarDecoded {
        addr: SocketAddr,
        small: Option<iced::widget::image::Handle>,
        large: Option<iced::widget::image::Handle>,
    },
    NoOp,
}

pub struct ScanSettings {
    pub(crate) java_ports: String,
    pub(crate) bedrock_ports: String,
    pub(crate) concurrency: String,
    pub(crate) timeout_ms: String,
    pub(crate) java_ports_error: bool,
    pub(crate) bedrock_ports_error: bool,
    pub(crate) query_enabled: bool,
    pub(crate) online_mode_check: bool,
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
            query_enabled: true,
            online_mode_check: false,
        }
    }
}

impl ScanSettings {
    fn java_ports_parsed(&self) -> Ports {
        Ports::from_input(&self.java_ports)
    }
    fn bedrock_ports_parsed(&self) -> Ports {
        Ports::from_input(&self.bedrock_ports)
    }
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
    pub(crate) theme_pref: crate::config::ThemePref,
    /// Resolved from `theme_pref`: for `System` it tracks the OS scheme reported
    /// by the color-scheme subscription; otherwise it is fixed by the preference.
    pub(crate) is_dark: bool,
    pub(crate) language: Language,
    pub(crate) copied: bool,
    /// Whether the version cell in the server preview dialog is expanded to show
    /// the full (possibly long) version list. Reset whenever the dialog reopens.
    pub(crate) version_expanded: bool,
    pub(crate) refresh_index: usize,
    /// Count of input lines rejected by the last "Add ranges" confirm; drives the
    /// warning in that dialog and is 0 while there is nothing to report.
    pub(crate) rejected_ranges: usize,
}

impl McScan {
    pub fn init() -> (Self, Task<Message>) {
        let cfg = crate::config::load().unwrap_or_default();

        let language =
            crate::config::language_from_code(&cfg.language).unwrap_or_else(Language::detect);
        let mut address_list = AddressList::default();
        address_list.push_ranges(parse_ip_ranges(&cfg.ranges.join("\n")));

        let theme_pref = crate::config::ThemePref::from_code(&cfg.theme);
        // `System` starts dark and is corrected by the color-scheme subscription's
        // first emission; the explicit prefs are already final.
        let is_dark = theme_pref != crate::config::ThemePref::Light;

        let app = Self {
            wid: None,
            results: ResultsList::default(),
            address_list,
            settings: ScanSettings {
                java_ports: cfg.java_ports,
                bedrock_ports: cfg.bedrock_ports,
                concurrency: cfg.concurrency,
                timeout_ms: cfg.timeout_ms,
                java_ports_error: false,
                bedrock_ports_error: false,
                query_enabled: cfg.query_enabled,
                online_mode_check: cfg.online_mode_check,
            },
            is_scanning: false,
            scan_id: 0,
            total_targets: 0,
            scanned_count: 0,
            modal: ModalKind::None,
            ranges_editor: iced::widget::text_editor::Content::new(),
            theme_pref,
            is_dark,
            language,
            copied: false,
            version_expanded: false,
            refresh_index: 0,
            rejected_ranges: 0,
        };
        (
            app,
            Task::discard(window::latest()).map(Message::WindowInitialized),
        )
    }

    /// Persist the settings that should survive a restart (ranges, ports, theme,
    /// language, toggles). Best-effort; called at natural boundaries rather than
    /// on every keystroke.
    fn persist(&self) {
        crate::config::save(&crate::config::Config {
            ranges: self
                .address_list
                .values()
                .iter()
                .map(|n| n.to_string())
                .collect(),
            java_ports: self.settings.java_ports.clone(),
            bedrock_ports: self.settings.bedrock_ports.clone(),
            concurrency: self.settings.concurrency.clone(),
            timeout_ms: self.settings.timeout_ms.clone(),
            query_enabled: self.settings.query_enabled,
            online_mode_check: self.settings.online_mode_check,
            theme: self.theme_pref.code().to_string(),
            language: crate::config::language_code(self.language).to_string(),
        });
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowInitialized(id) => {
                self.wid = id;
            }

            Message::ScanStart => {
                let jp = self.settings.java_ports_parsed();
                let bp = self.settings.bedrock_ports_parsed();
                self.settings.java_ports_error = jp.is_empty();
                self.settings.bedrock_ports_error = bp.is_empty();

                if self.address_list.values().is_empty() {
                    self.ranges_editor = iced::widget::text_editor::Content::new();
                    self.rejected_ranges = 0;
                    self.modal = ModalKind::AddRanges;
                    return Task::none();
                }
                if jp.is_empty() && bp.is_empty() {
                    self.modal = ModalKind::Settings;
                    return Task::none();
                }

                self.results.clear();
                let config = self.scan_config();
                self.total_targets = config.target_count();
                self.scanned_count = 0;
                self.scan_id += 1;
                self.is_scanning = true;
                tracing::info!(
                    targets = self.total_targets,
                    concurrency = config.concurrency.get(),
                    timeout_ms = config.timeout_ms.get(),
                    "scan started"
                );
                self.persist();
            }

            Message::ScanStop => {
                self.is_scanning = false;
                tracing::info!(
                    scanned = self.scanned_count,
                    found = self.results.count(),
                    "scan stopped"
                );
            }

            Message::ServerFound(info) => {
                let addr = info.addr;
                if let Some(f) = self.results.push(info) {
                    return self.spawn_favicon_decode(addr, f);
                }
            }

            Message::ScanProgress(n) => {
                if self.is_scanning {
                    self.scanned_count = n;
                }
            }

            Message::ScanComplete => {
                self.scanned_count = self.total_targets;
                self.is_scanning = false;
                tracing::info!(
                    targets = self.total_targets,
                    found = self.results.count(),
                    "scan complete"
                );
            }

            Message::AddressList(msg) => {
                let ranges_changed = matches!(msg, AddressListMessage::RemoveClicked(_));
                self.address_list.update(msg);
                if ranges_changed {
                    self.persist();
                }
            }

            Message::ResultsList(msg) => match msg {
                ResultsListMessage::OpenPreview(addr) => {
                    self.modal = ModalKind::ServerPreview(addr);
                    self.copied = false;
                    self.version_expanded = false;
                    if let Some(server) = self.results.get_by_addr(addr) {
                        let edition = server.edition.clone();
                        return self.spawn_probe(addr, edition);
                    }
                }
            },

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
            Message::ToggleQuery(v) => self.settings.query_enabled = v,
            Message::ToggleOnlineModeCheck(v) => self.settings.online_mode_check = v,

            Message::OpenModal(kind) => {
                if kind == ModalKind::AddRanges {
                    self.rejected_ranges = 0;
                }
                self.modal = kind;
            }
            Message::CloseModal => {
                self.modal = ModalKind::None;
                self.copied = false;
                self.persist();
            }

            Message::RangesEditorAction(action) => self.ranges_editor.perform(action),

            Message::ConfirmAddRanges => {
                let raw = self.ranges_editor.text();
                let (ranges, rejected) = parse_ip_ranges_reporting(&raw);
                self.address_list.push_ranges(ranges);
                self.rejected_ranges = rejected.len();
                if rejected.is_empty() {
                    self.ranges_editor = iced::widget::text_editor::Content::new();
                    self.modal = ModalKind::None;
                } else {
                    // Keep the dialog open with only the unparsed lines so the
                    // user can see and fix them; the valid ones were added.
                    self.ranges_editor =
                        iced::widget::text_editor::Content::with_text(&rejected.join("\n"));
                }
                self.persist();
            }

            Message::SetThemePref(pref) => {
                self.theme_pref = pref;
                match pref {
                    crate::config::ThemePref::Dark => self.is_dark = true,
                    crate::config::ThemePref::Light => self.is_dark = false,
                    // Leave is_dark until the subscription reports the OS scheme.
                    crate::config::ThemePref::System => {}
                }
                self.persist();
            }

            Message::SystemColorScheme(dark) => {
                tracing::debug!(dark, "OS color scheme");
                if self.theme_pref == crate::config::ThemePref::System {
                    self.is_dark = dark;
                }
            }
            Message::SetLanguage(lang) => {
                self.language = lang;
                self.persist();
            }

            Message::CopyAddress => {
                if let ModalKind::ServerPreview(addr) = &self.modal {
                    let s = format!("{}:{}", addr.ip(), addr.port());
                    self.copied = true;
                    let (tx, rx) = oneshot::channel::<()>();
                    std::thread::spawn(move || {
                        std::thread::sleep(Duration::from_millis(1500));
                        let _ = tx.send(());
                    });
                    return Task::batch([
                        iced::clipboard::write(s),
                        Task::perform(
                            async move {
                                let _ = rx.await;
                            },
                            |_| Message::CopiedReset,
                        ),
                    ]);
                }
            }

            Message::CopiedReset => {
                self.copied = false;
            }

            Message::ToggleVersionExpand => {
                self.version_expanded = !self.version_expanded;
            }

            Message::ExportResults => {
                if self.results.count() > 0 {
                    let csv = crate::export::to_csv(self.results.items());
                    // Drive the dialog future to completion off the UI thread.
                    RUNTIME.spawn(crate::export::save_dialog(csv));
                }
            }

            Message::RefreshTick => {
                let count = self.results.count();
                if count == 0 {
                    return Task::none();
                }
                let idx = self.refresh_index % count;
                self.refresh_index = self.refresh_index.wrapping_add(1);
                let addr = self.results.items()[idx].addr;
                let edition = self.results.items()[idx].edition.clone();
                return self.spawn_probe(addr, edition);
            }

            Message::ServerRefreshed(Some(info)) => {
                let addr = info.addr;
                if let Some(f) = self.results.refresh(info) {
                    return self.spawn_favicon_decode(addr, f);
                }
            }

            Message::ServerRefreshed(None) => {}

            Message::AvatarDecoded { addr, small, large } => {
                self.results.set_avatars(addr, small, large);
            }

            Message::NoOp => {}
        }

        Task::none()
    }

    pub fn tr(&self) -> &'static Tr {
        i18n::tr(self.language)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let scan_sub = if self.is_scanning {
            let config = Arc::new(self.scan_config());
            Subscription::run_with(
                ScanKey {
                    id: self.scan_id,
                    config,
                },
                build_scan_stream,
            )
        } else {
            Subscription::none()
        };

        let refresh_sub = if self.results.count() > 0 {
            Subscription::run_with(REFRESH_TIMER_ID, refresh_timer_stream)
        } else {
            Subscription::none()
        };

        // Only follow the OS scheme while the user is on "System".
        let theme_sub = if self.theme_pref == crate::config::ThemePref::System {
            Subscription::run_with(THEME_SUB_ID, system_color_scheme_stream)
        } else {
            Subscription::none()
        };

        Subscription::batch([scan_sub, refresh_sub, theme_sub])
    }

    pub fn theme(&self) -> Theme {
        if self.is_dark {
            COLOR_THEME.clone()
        } else {
            COLOR_THEME_LIGHT.clone()
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        use crate::components::{left_panel, results_list, right_panel};
        use crate::styles::{c, is_dark};
        use iced::widget::{Space, Stack, container, row};
        use iced::{Fill, Length::Fixed};

        let sep = container(Space::new())
            .width(Fixed(1.0))
            .height(Fill)
            .style(|t: &iced::Theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(if is_dark(t) {
                    c("#1A1F27")
                } else {
                    c("#E1E5EA")
                })),
                ..Default::default()
            });

        let base = container(
            row![
                container(left_panel::render(self)).width(Fill).height(Fill),
                sep,
                container(right_panel::render(self))
                    .width(Fixed(340.0))
                    .height(Fill),
            ]
            .width(Fill)
            .height(Fill),
        )
        .style(app_bg_style)
        .width(Fill)
        .height(Fill);

        let mut stack = Stack::new().push(base);
        stack = match &self.modal {
            ModalKind::None => stack,
            ModalKind::Settings => stack.push(settings::render(self)),
            ModalKind::AddRanges => stack.push(address_list::add_dialog::render(self)),
            ModalKind::ServerPreview(_) => stack.push(results_list::preview_dialog::render(self)),
        };
        stack.into()
    }

    fn spawn_probe(&self, addr: SocketAddr, edition: scanner::types::Edition) -> Task<Message> {
        let timeout = TimeoutMs::from_input(&self.settings.timeout_ms).get();
        let query_enabled = self.settings.query_enabled;
        let online_mode_check = self.settings.online_mode_check;
        let (tx, rx) = oneshot::channel::<Option<ServerInfo>>();
        RUNTIME.spawn(async move {
            let result =
                scanner::probe_server(addr, edition, timeout, query_enabled, online_mode_check)
                    .await;
            let _ = tx.send(result);
        });
        Task::perform(
            async move { rx.await.ok().flatten() },
            Message::ServerRefreshed,
        )
    }

    fn spawn_favicon_decode(&self, addr: SocketAddr, favicon: String) -> Task<Message> {
        let (tx, rx) = oneshot::channel();
        RUNTIME.spawn_blocking(move || {
            let _ = tx.send(crate::components::results_list::decode_favicon_avatars(
                &favicon,
            ));
        });
        Task::perform(async move { rx.await.ok() }, move |res| match res {
            Some((small, large)) => Message::AvatarDecoded { addr, small, large },
            None => Message::NoOp,
        })
    }

    fn scan_config(&self) -> ScanConfig {
        ScanConfig {
            ranges: self.address_list.values().to_vec(),
            java_ports: self.settings.java_ports_parsed(),
            bedrock_ports: self.settings.bedrock_ports_parsed(),
            concurrency: Concurrency::from_input(&self.settings.concurrency),
            timeout_ms: TimeoutMs::from_input(&self.settings.timeout_ms),
        }
    }
}

fn app_bg_style(t: &iced::Theme) -> iced::widget::container::Style {
    use crate::styles::{c, is_dark};
    iced::widget::container::Style {
        background: Some(iced::Background::Color(if is_dark(t) {
            c("#0E1116")
        } else {
            c("#F0F1F3")
        })),
        ..Default::default()
    }
}

fn refresh_timer_stream(_: &u8) -> BoxStream<'static, Message> {
    let (tx, rx) = mpsc::unbounded();
    RUNTIME.spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(30)).await;
            if tx.unbounded_send(Message::RefreshTick).is_err() {
                break;
            }
        }
    });
    Box::pin(rx)
}

// Emits the OS color scheme now and whenever it changes. `NoPreference` (unknown
// or unset) is treated as dark to match the app's dark-first palette.
fn system_color_scheme_stream(_: &u8) -> BoxStream<'static, Message> {
    use mundy::{ColorScheme, Interest, Preferences};
    Box::pin(
        Preferences::stream(Interest::ColorScheme)
            .map(|p| Message::SystemColorScheme(p.color_scheme != ColorScheme::Light)),
    )
}

struct ScanKey {
    id: u64,
    config: Arc<ScanConfig>,
}

// Subscription identity is the id alone; the config must not be hashed.
impl std::hash::Hash for ScanKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn build_scan_stream(key: &ScanKey) -> BoxStream<'static, Message> {
    let config = key.config.clone();
    let (tx, rx) = mpsc::unbounded();

    // Runs on the shared runtime rather than spawning a fresh OS thread + tokio
    // runtime per scan. The task ends on its own when the subscription is dropped
    // (the receiver goes away and the sends start failing).
    RUNTIME.spawn(async move {
        let mut stream = Box::pin(scanner::scan(config));
        let mut scanned = 0usize;
        while let Some(maybe_info) = stream.next().await {
            scanned += 1;
            if let Some(info) = maybe_info
                && tx.unbounded_send(Message::ServerFound(info)).is_err()
            {
                return;
            }
            if scanned.is_multiple_of(512)
                && tx.unbounded_send(Message::ScanProgress(scanned)).is_err()
            {
                return;
            }
        }
        let _ = tx.unbounded_send(Message::ScanComplete);
    });

    Box::pin(rx)
}
