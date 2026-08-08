mod avatar;
mod item;
pub mod preview_dialog;
mod toolbar;
mod virtual_list;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::net::SocketAddr;

use iced::widget::{button, container, image, text};
use iced::{Background, Border, Element, Fill, Padding, Shadow, Theme};

use crate::i18n::Tr;
use crate::styles::{SANS, c, is_dark};
use crate::text::strip_section_codes;
use scanner::types::{Edition, ServerInfo};

use avatar::{AvatarSize, favicon_handle};
use item::{CARD_HEIGHT, server_card_content};
use virtual_list::VirtualList;

const LIST_PADDING: Padding = Padding {
    top: 12.0,
    right: 16.0,
    bottom: 12.0,
    left: 16.0,
};
const CARD_SPACING: f32 = 9.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortKey {
    #[default]
    Found,
    Players,
    Ping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditionFilter {
    #[default]
    All,
    Java,
    Bedrock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OnlineModeFilter {
    #[default]
    Any,
    Online,
    Cracked,
}

#[derive(Default)]
pub(super) struct Filters {
    pub(super) query: String,
    pub(super) sort: SortKey,
    pub(super) descending: bool,
    pub(super) edition: EditionFilter,
    pub(super) online_mode: OnlineModeFilter,
    pub(super) version: String,
    pub(super) plugin: String,
}

impl Filters {
    pub(super) fn active_count(&self) -> usize {
        usize::from(self.edition != EditionFilter::All)
            + usize::from(self.online_mode != OnlineModeFilter::Any)
            + usize::from(!self.version.trim().is_empty())
            + usize::from(!self.plugin.trim().is_empty())
    }
}

#[derive(Default)]
pub struct ResultsList {
    items: Vec<ServerInfo>,
    // Indices stay valid because items are only appended or updated, never removed.
    index: HashMap<SocketAddr, usize>,
    favicon_hash: HashMap<SocketAddr, u64>,
    avatars_small: HashMap<SocketAddr, image::Handle>,
    avatars_large: HashMap<SocketAddr, image::Handle>,
    filters: Filters,
    query_input: String,
    // Bumped per keystroke; a debounced apply only runs when its generation still matches.
    search_gen: u64,
    sort_open: bool,
    filters_open: bool,
    view_order: RefCell<Vec<usize>>,
    view_dirty: Cell<bool>,
}

#[derive(Debug, Clone)]
pub enum ResultsListMessage {
    OpenPreview(SocketAddr),
    SearchInput(String),
    SearchApply(u64),
    ToggleSortMenu,
    ToggleFilterMenu,
    DismissMenus,
    SortPicked(SortKey),
    SortDescending(bool),
    EditionPicked(EditionFilter),
    OnlineModePicked(OnlineModeFilter),
    VersionFilter(String),
    PluginFilter(String),
    ResetFilters,
}

impl ResultsList {
    pub fn push(&mut self, mut info: ServerInfo) -> Option<String> {
        let addr = info.addr;
        let favicon = info.favicon.take();
        match self.index.get(&addr) {
            Some(&idx) => self.items[idx] = info,
            None => {
                self.index.insert(addr, self.items.len());
                self.items.push(info);
            }
        }
        self.view_dirty.set(true);
        self.update_favicon(addr, favicon)
    }
    pub fn clear(&mut self) {
        self.items.clear();
        self.index.clear();
        self.favicon_hash.clear();
        self.avatars_small.clear();
        self.avatars_large.clear();
        self.view_dirty.set(true);
    }
    pub fn count(&self) -> usize {
        self.items.len()
    }
    pub fn visible_count(&self) -> usize {
        self.ensure_view();
        self.view_order.borrow().len()
    }
    pub fn items(&self) -> &[ServerInfo] {
        &self.items
    }

    pub fn get_by_addr(&self, addr: SocketAddr) -> Option<&ServerInfo> {
        self.index.get(&addr).map(|&idx| &self.items[idx])
    }

    pub fn avatar_large(&self, addr: SocketAddr) -> Option<image::Handle> {
        self.avatars_large.get(&addr).cloned()
    }

    pub fn set_avatars(
        &mut self,
        addr: SocketAddr,
        small: Option<image::Handle>,
        large: Option<image::Handle>,
    ) {
        match small {
            Some(h) => {
                self.avatars_small.insert(addr, h);
            }
            None => {
                self.avatars_small.remove(&addr);
            }
        }
        match large {
            Some(h) => {
                self.avatars_large.insert(addr, h);
            }
            None => {
                self.avatars_large.remove(&addr);
            }
        }
    }

    pub fn refresh(&mut self, mut info: ServerInfo) -> Option<String> {
        let addr = info.addr;
        let favicon = info.favicon.take();
        {
            let &idx = self.index.get(&addr)?;
            let s = &mut self.items[idx];
            s.online = info.online;
            s.max_players = info.max_players;
            s.latency_ms = info.latency_ms;
            s.samples = info.samples;
            s.sample_ids = info.sample_ids;
            s.mods = info.mods;
            s.secure_chat = info.secure_chat;
            s.gamemode = info.gamemode;
            s.world = info.world;
            s.plugins = info.plugins;
            s.online_mode = info.online_mode;
            s.ping_history.push(info.latency_ms);
            if s.ping_history.len() > 30 {
                s.ping_history.remove(0);
            }
        }
        self.view_dirty.set(true);
        self.update_favicon(addr, favicon)
    }

    fn update_favicon(&mut self, addr: SocketAddr, favicon: Option<String>) -> Option<String> {
        let new_hash = favicon.as_deref().map(favicon_hash);
        if self.favicon_hash.get(&addr).copied() == new_hash {
            return None;
        }
        match new_hash {
            Some(h) => {
                self.favicon_hash.insert(addr, h);
            }
            None => {
                self.favicon_hash.remove(&addr);
            }
        }
        self.avatars_small.remove(&addr);
        self.avatars_large.remove(&addr);
        favicon
    }

    pub fn set_search_input(&mut self, text: String) -> u64 {
        self.query_input = text;
        self.search_gen = self.search_gen.wrapping_add(1);
        self.search_gen
    }

    pub fn apply_search(&mut self, generation: u64) {
        if generation == self.search_gen && self.filters.query != self.query_input {
            self.filters.query = self.query_input.clone();
            self.view_dirty.set(true);
        }
    }

    pub fn set_sort(&mut self, key: SortKey) {
        self.filters.sort = key;
        self.filters.descending = key == SortKey::Players;
        self.view_dirty.set(true);
    }

    pub fn set_sort_descending(&mut self, descending: bool) {
        self.filters.descending = descending;
        self.view_dirty.set(true);
    }

    pub fn set_edition(&mut self, edition: EditionFilter) {
        self.filters.edition = edition;
        self.view_dirty.set(true);
    }

    pub fn set_online_mode(&mut self, mode: OnlineModeFilter) {
        self.filters.online_mode = mode;
        self.view_dirty.set(true);
    }

    pub fn set_version_filter(&mut self, text: String) {
        self.filters.version = text;
        self.view_dirty.set(true);
    }

    pub fn set_plugin_filter(&mut self, text: String) {
        self.filters.plugin = text;
        self.view_dirty.set(true);
    }

    pub fn reset_filters(&mut self) {
        self.filters.edition = EditionFilter::All;
        self.filters.online_mode = OnlineModeFilter::Any;
        self.filters.version.clear();
        self.filters.plugin.clear();
        self.view_dirty.set(true);
    }

    pub fn toggle_sort_menu(&mut self) {
        self.sort_open = !self.sort_open;
        self.filters_open = false;
    }

    pub fn toggle_filter_menu(&mut self) {
        self.filters_open = !self.filters_open;
        self.sort_open = false;
    }

    pub fn close_menus(&mut self) {
        self.sort_open = false;
        self.filters_open = false;
    }

    fn ensure_view(&self) {
        if !self.view_dirty.get() {
            return;
        }
        let f = &self.filters;
        let q = f.query.trim().to_lowercase();
        let version_q = f.version.trim().to_lowercase();
        let plugin_q = f.plugin.trim().to_lowercase();
        let mut order: Vec<usize> = (0..self.items.len())
            .filter(|&i| passes_filters(&self.items[i], f, &q, &version_q, &plugin_q))
            .collect();
        // Stable sort keeps ties in discovery order.
        match f.sort {
            SortKey::Found => {}
            SortKey::Players => order.sort_by_key(|&i| self.items[i].online),
            SortKey::Ping => order.sort_by_key(|&i| self.items[i].latency_ms),
        }
        if f.descending {
            order.reverse();
        }
        *self.view_order.borrow_mut() = order;
        self.view_dirty.set(false);
    }

    pub fn toolbar(&self, tr: &'static Tr) -> Element<'_, ResultsListMessage> {
        toolbar::render(self, tr)
    }

    pub fn view(&self, tr: &'static Tr) -> Element<'_, ResultsListMessage> {
        if self.items.is_empty() {
            return empty_state(tr.results_empty);
        }

        self.ensure_view();
        let order: Vec<usize> = self.view_order.borrow().clone();
        if order.is_empty() {
            return empty_state(tr.no_matches);
        }

        VirtualList::new(
            order.len(),
            CARD_HEIGHT,
            CARD_SPACING,
            LIST_PADDING,
            move |row| {
                let info = &self.items[order[row]];
                let addr = info.addr;
                let content = server_card_content(info, self.avatars_small.get(&addr).cloned(), tr);
                button(content)
                    .on_press(ResultsListMessage::OpenPreview(addr))
                    .style(card_btn_style)
                    .padding(Padding::from([13, 15]))
                    .width(Fill)
                    .height(CARD_HEIGHT)
                    .into()
            },
        )
        .into()
    }
}

fn empty_state(message: &str) -> Element<'_, ResultsListMessage> {
    container(
        text(message)
            .size(14)
            .font(SANS)
            .style(|t: &Theme| text::Style {
                color: Some(if is_dark(t) {
                    c("#5C636F")
                } else {
                    c("#A0A7B1")
                }),
            }),
    )
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

// The `*_q` args are pre-lowered, pre-trimmed queries (empty = facet disabled).
fn passes_filters(
    s: &ServerInfo,
    f: &Filters,
    query: &str,
    version_q: &str,
    plugin_q: &str,
) -> bool {
    edition_matches(f.edition, &s.edition)
        && online_mode_matches(f.online_mode, s.online_mode)
        && (query.is_empty() || search_matches(s, query))
        && (version_q.is_empty()
            || strip_section_codes(&s.version)
                .to_lowercase()
                .contains(version_q))
        && (plugin_q.is_empty()
            || s.plugins
                .iter()
                .any(|p| strip_section_codes(p).to_lowercase().contains(plugin_q)))
}

fn edition_matches(filter: EditionFilter, edition: &Edition) -> bool {
    match filter {
        EditionFilter::All => true,
        EditionFilter::Java => *edition == Edition::Java,
        EditionFilter::Bedrock => *edition == Edition::Bedrock,
    }
}

fn online_mode_matches(filter: OnlineModeFilter, mode: Option<bool>) -> bool {
    match filter {
        OnlineModeFilter::Any => true,
        OnlineModeFilter::Online => mode == Some(true),
        OnlineModeFilter::Cracked => mode == Some(false),
    }
}

fn search_matches(s: &ServerInfo, query: &str) -> bool {
    let addr = format!("{}:{}", s.addr.ip(), s.addr.port());
    addr.contains(query)
        || strip_section_codes(&s.motd).to_lowercase().contains(query)
        || strip_section_codes(&s.version)
            .to_lowercase()
            .contains(query)
}

fn card_btn_style(t: &Theme, status: button::Status) -> button::Style {
    let dark = is_dark(t);
    let bg = if dark { c("#181D25") } else { c("#FFFFFF") };
    let bg_hover = if dark { c("#1E2530") } else { c("#F4F7FA") };
    let bg_press = if dark { c("#232A34") } else { c("#EAF0F5") };
    let border_n = if dark { c("#232A34") } else { c("#E5E9EF") };
    let border_h = if dark { c("#2E3849") } else { c("#C8D0DA") };
    let txt = if dark { c("#E8EBF0") } else { c("#161A20") };

    let base = button::Style {
        background: Some(Background::Color(bg)),
        text_color: txt,
        border: Border {
            color: border_n,
            width: 1.0,
            radius: 10.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(bg_hover)),
            border: Border {
                color: border_h,
                width: 1.0,
                radius: 10.0.into(),
            },
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(bg_press)),
            ..base
        },
        _ => base,
    }
}

pub(crate) fn parse_version(raw: &str) -> (Option<String>, String) {
    if let Some(pos) = raw.find(' ') {
        let prefix = &raw[..pos];
        let rest = raw[pos + 1..].trim();
        if !prefix.is_empty()
            && prefix
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_alphabetic())
        {
            return (Some(prefix.to_string()), rest.to_string());
        }
    }
    (None, raw.to_string())
}

fn favicon_hash(favicon: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    favicon.hash(&mut hasher);
    hasher.finish()
}

pub(crate) fn decode_favicon_avatars(
    favicon: &str,
) -> (Option<image::Handle>, Option<image::Handle>) {
    (
        favicon_handle(favicon, AvatarSize::SMALL),
        favicon_handle(favicon, AvatarSize::LARGE),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use scanner::types::Edition;

    fn addr(port: u16) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], port))
    }

    #[test]
    fn push_dedups_by_addr_and_keeps_indices_lookupable() {
        let mut list = ResultsList::default();
        list.push(ServerInfo::base(addr(25565), Edition::Java));
        list.push(ServerInfo::base(addr(25566), Edition::Java));

        let mut updated = ServerInfo::base(addr(25565), Edition::Java);
        updated.online = 42;
        list.push(updated);

        assert_eq!(list.count(), 2);
        assert_eq!(list.get_by_addr(addr(25565)).unwrap().online, 42);
        assert_eq!(list.get_by_addr(addr(25566)).unwrap().online, 0);
        assert!(list.get_by_addr(addr(25567)).is_none());
    }

    #[test]
    fn refresh_updates_existing_entry_via_index() {
        let mut list = ResultsList::default();
        list.push(ServerInfo::base(addr(25565), Edition::Java));

        let mut info = ServerInfo::base(addr(25565), Edition::Java);
        info.latency_ms = 7;
        assert!(list.refresh(info).is_none());
        assert_eq!(list.get_by_addr(addr(25565)).unwrap().latency_ms, 7);

        assert!(
            list.refresh(ServerInfo::base(addr(25599), Edition::Java))
                .is_none()
        );
        assert_eq!(list.count(), 1);
    }

    #[test]
    fn push_hands_off_favicon_once_and_never_retains_it() {
        let mut list = ResultsList::default();
        let mut info = ServerInfo::base(addr(25565), Edition::Java);
        info.favicon = Some("data:image/png;base64,AAAA".into());

        assert_eq!(
            list.push(info).as_deref(),
            Some("data:image/png;base64,AAAA")
        );
        assert!(list.get_by_addr(addr(25565)).unwrap().favicon.is_none());

        let mut same = ServerInfo::base(addr(25565), Edition::Java);
        same.favicon = Some("data:image/png;base64,AAAA".into());
        assert!(list.push(same).is_none());

        let mut changed = ServerInfo::base(addr(25565), Edition::Java);
        changed.favicon = Some("data:image/png;base64,BBBB".into());
        assert_eq!(
            list.push(changed).as_deref(),
            Some("data:image/png;base64,BBBB")
        );
    }

    #[test]
    fn clear_drops_the_index() {
        let mut list = ResultsList::default();
        list.push(ServerInfo::base(addr(25565), Edition::Java));
        list.clear();
        assert!(list.get_by_addr(addr(25565)).is_none());
        assert_eq!(list.count(), 0);
    }

    fn server(port: u16, edition: Edition, online: u32, latency: u64, motd: &str) -> ServerInfo {
        let mut s = ServerInfo::base(addr(port), edition);
        s.online = online;
        s.latency_ms = latency;
        s.motd = motd.into();
        s
    }

    fn visible_ports(list: &ResultsList) -> Vec<u16> {
        list.ensure_view();
        list.view_order
            .borrow()
            .iter()
            .map(|&i| list.items[i].addr.port())
            .collect()
    }

    fn seeded() -> ResultsList {
        let mut list = ResultsList::default();
        list.push(server(1, Edition::Java, 10, 200, "Alpha survival"));
        list.push(server(2, Edition::Bedrock, 50, 20, "Beta creative"));
        list.push(server(3, Edition::Java, 5, 90, "Gamma"));
        list
    }

    fn search(list: &mut ResultsList, q: &str) {
        let generation = list.set_search_input(q.into());
        list.apply_search(generation);
    }

    #[test]
    fn default_view_keeps_discovery_order_and_counts_all() {
        let list = seeded();
        assert_eq!(visible_ports(&list), vec![1, 2, 3]);
        assert_eq!(list.visible_count(), 3);
        assert_eq!(list.count(), 3);
    }

    #[test]
    fn search_matches_motd_and_address_case_insensitively() {
        let mut list = seeded();
        search(&mut list, "beta");
        assert_eq!(visible_ports(&list), vec![2]);

        search(&mut list, "127.0.0.1:3");
        assert_eq!(visible_ports(&list), vec![3]);

        search(&mut list, "  ");
        assert_eq!(visible_ports(&list), vec![1, 2, 3]);
    }

    #[test]
    fn stale_debounced_search_is_ignored() {
        let mut list = seeded();
        let stale = list.set_search_input("beta".into());
        let _fresh = list.set_search_input("gamma".into());
        list.apply_search(stale);
        assert_eq!(visible_ports(&list), vec![1, 2, 3]);
    }

    #[test]
    fn online_mode_and_plugin_filters_narrow_results() {
        let mut list = ResultsList::default();
        let mut a = server(1, Edition::Java, 1, 10, "A");
        a.online_mode = Some(true);
        a.plugins = vec!["EssentialsX".into()];
        let mut b = server(2, Edition::Java, 1, 10, "B");
        b.online_mode = Some(false);
        list.push(a);
        list.push(b);

        list.set_online_mode(OnlineModeFilter::Cracked);
        assert_eq!(visible_ports(&list), vec![2]);

        list.set_online_mode(OnlineModeFilter::Any);
        list.set_plugin_filter("essentials".into());
        assert_eq!(visible_ports(&list), vec![1]);
        assert_eq!(list.filters.active_count(), 1);
    }

    #[test]
    fn edition_filter_narrows_to_one_edition() {
        let mut list = seeded();
        list.set_edition(EditionFilter::Java);
        assert_eq!(visible_ports(&list), vec![1, 3]);
        assert_eq!(list.visible_count(), 2);
        assert_eq!(list.count(), 3);
    }

    #[test]
    fn sort_players_defaults_to_high_first_and_direction_flips() {
        let mut list = seeded();
        list.set_sort(SortKey::Players);
        assert_eq!(visible_ports(&list), vec![2, 1, 3]);

        list.set_sort_descending(false);
        assert_eq!(visible_ports(&list), vec![3, 1, 2]);
    }

    #[test]
    fn sort_ping_defaults_to_low_first() {
        let mut list = seeded();
        list.set_sort(SortKey::Ping);
        assert_eq!(visible_ports(&list), vec![2, 3, 1]);
    }

    #[test]
    fn search_and_sort_compose() {
        let mut list = seeded();
        list.push(server(4, Edition::Java, 99, 10, "Alpha raid"));
        search(&mut list, "alpha");
        list.set_sort(SortKey::Players);
        assert_eq!(visible_ports(&list), vec![4, 1]);
    }
}
