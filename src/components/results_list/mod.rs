mod avatar;
mod item;
pub mod preview_dialog;
mod virtual_list;

use std::collections::HashMap;
use std::net::SocketAddr;

use iced::widget::{button, container, image, text};
use iced::{Background, Border, Element, Fill, Padding, Shadow, Theme};

use crate::i18n::Tr;
use crate::scanner::types::ServerInfo;
use crate::styles::{c, is_dark, SANS};

use avatar::{favicon_handle, AvatarSize};
use item::{server_card_content, CARD_HEIGHT};
use virtual_list::VirtualList;

const LIST_PADDING: Padding = Padding { top: 12.0, right: 16.0, bottom: 12.0, left: 16.0 };
const CARD_SPACING: f32 = 9.0;

#[derive(Default)]
pub struct ResultsList {
    items: Vec<ServerInfo>,
    /// Maps a server address to its slot in `items` for O(1) lookup and dedup.
    /// Items are only ever appended or updated in place (never removed), so the
    /// stored indices stay valid until `clear`.
    index: HashMap<SocketAddr, usize>,
    /// Hash of the last-seen raw favicon per address. The full base64 data-URI
    /// (dozens of KB) is decoded into the avatar handles below and then dropped;
    /// only this hash is retained, so a refresh can tell whether the favicon
    /// changed without holding the raw string alongside the decoded pixels.
    favicon_hash: HashMap<SocketAddr, u64>,
    /// Favicon cache
    avatars_small: HashMap<SocketAddr, image::Handle>,
    avatars_large: HashMap<SocketAddr, image::Handle>,
}

#[derive(Debug, Clone)]
pub enum ResultsListMessage {
    OpenPreview(SocketAddr),
}

impl ResultsList {
    /// Stores `info`, returning the raw favicon to decode when it differs from
    /// what was last seen for this address (so the caller decodes it off-thread
    /// and the list never retains the base64 string).
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
        self.update_favicon(addr, favicon)
    }
    pub fn clear(&mut self) {
        self.items.clear();
        self.index.clear();
        self.favicon_hash.clear();
        self.avatars_small.clear();
        self.avatars_large.clear();
    }
    pub fn count(&self) -> usize { self.items.len() }
    pub fn items(&self) -> &[ServerInfo] { &self.items }

    pub fn get_by_addr(&self, addr: SocketAddr) -> Option<&ServerInfo> {
        self.index.get(&addr).map(|&idx| &self.items[idx])
    }

    pub fn avatar_large(&self, addr: SocketAddr) -> Option<image::Handle> {
        self.avatars_large.get(&addr).cloned()
    }

    pub fn set_avatars(&mut self, addr: SocketAddr, small: Option<image::Handle>, large: Option<image::Handle>) {
        match small {
            Some(h) => { self.avatars_small.insert(addr, h); }
            None => { self.avatars_small.remove(&addr); }
        }
        match large {
            Some(h) => { self.avatars_large.insert(addr, h); }
            None => { self.avatars_large.remove(&addr); }
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
        self.update_favicon(addr, favicon)
    }

    /// Records the favicon's hash and drops avatars that no longer match,
    /// returning the raw data-URI only when it changed so it is decoded once.
    fn update_favicon(&mut self, addr: SocketAddr, favicon: Option<String>) -> Option<String> {
        let new_hash = favicon.as_deref().map(favicon_hash);
        if self.favicon_hash.get(&addr).copied() == new_hash {
            return None;
        }
        match new_hash {
            Some(h) => { self.favicon_hash.insert(addr, h); }
            None => { self.favicon_hash.remove(&addr); }
        }
        self.avatars_small.remove(&addr);
        self.avatars_large.remove(&addr);
        favicon
    }

    pub fn view(&self, tr: &'static Tr) -> Element<'_, ResultsListMessage> {
        if self.items.is_empty() {
            return container(
                text(tr.results_empty)
                    .size(14)
                    .font(SANS)
                    .style(|t: &Theme| text::Style {
                        color: Some(if is_dark(t) { c("#5C636F") } else { c("#A0A7B1") }),
                    }),
            )
            .center_x(Fill)
            .center_y(Fill)
            .into();
        }

        VirtualList::new(
            self.items.len(),
            CARD_HEIGHT,
            CARD_SPACING,
            LIST_PADDING,
            move |i| {
                let info = &self.items[i];
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

fn card_btn_style(t: &Theme, status: button::Status) -> button::Style {
    let dark = is_dark(t);
    let bg       = if dark { c("#181D25") } else { c("#FFFFFF") };
    let bg_hover = if dark { c("#1E2530") } else { c("#F4F7FA") };
    let bg_press = if dark { c("#232A34") } else { c("#EAF0F5") };
    let border_n = if dark { c("#232A34") } else { c("#E5E9EF") };
    let border_h = if dark { c("#2E3849") } else { c("#C8D0DA") };
    let txt      = if dark { c("#E8EBF0") } else { c("#161A20") };

    let base = button::Style {
        background: Some(Background::Color(bg)),
        text_color: txt,
        border: Border { color: border_n, width: 1.0, radius: 10.0.into() },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(bg_hover)),
            border: Border { color: border_h, width: 1.0, radius: 10.0.into() },
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
        if !prefix.is_empty() && prefix.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
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

pub(crate) fn decode_favicon_avatars(favicon: &str) -> (Option<image::Handle>, Option<image::Handle>) {
    (
        favicon_handle(favicon, AvatarSize::SMALL),
        favicon_handle(favicon, AvatarSize::LARGE),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::types::Edition;

    fn addr(port: u16) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], port))
    }

    #[test]
    fn push_dedups_by_addr_and_keeps_indices_lookupable() {
        let mut list = ResultsList::default();
        list.push(ServerInfo::base(addr(25565), Edition::Java));
        list.push(ServerInfo::base(addr(25566), Edition::Java));

        // Re-pushing the same address updates in place instead of appending.
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
        assert!(list.refresh(info).is_none()); // favicon unchanged → nothing to decode
        assert_eq!(list.get_by_addr(addr(25565)).unwrap().latency_ms, 7);

        // Refreshing an unknown address is a no-op.
        assert!(list.refresh(ServerInfo::base(addr(25599), Edition::Java)).is_none());
        assert_eq!(list.count(), 1);
    }

    #[test]
    fn push_hands_off_favicon_once_and_never_retains_it() {
        let mut list = ResultsList::default();
        let mut info = ServerInfo::base(addr(25565), Edition::Java);
        info.favicon = Some("data:image/png;base64,AAAA".into());

        // First sighting hands the raw favicon off for decoding...
        assert_eq!(list.push(info).as_deref(), Some("data:image/png;base64,AAAA"));
        // ...but the stored item does not keep the base64 string.
        assert!(list.get_by_addr(addr(25565)).unwrap().favicon.is_none());

        // Re-pushing the same favicon is a no-op (already decoded).
        let mut same = ServerInfo::base(addr(25565), Edition::Java);
        same.favicon = Some("data:image/png;base64,AAAA".into());
        assert!(list.push(same).is_none());

        // A changed favicon is handed off again.
        let mut changed = ServerInfo::base(addr(25565), Edition::Java);
        changed.favicon = Some("data:image/png;base64,BBBB".into());
        assert_eq!(list.push(changed).as_deref(), Some("data:image/png;base64,BBBB"));
    }

    #[test]
    fn clear_drops_the_index() {
        let mut list = ResultsList::default();
        list.push(ServerInfo::base(addr(25565), Edition::Java));
        list.clear();
        assert!(list.get_by_addr(addr(25565)).is_none());
        assert_eq!(list.count(), 0);
    }
}
