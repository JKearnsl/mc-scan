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
    /// Favicon cache
    avatars_small: HashMap<SocketAddr, image::Handle>,
    avatars_large: HashMap<SocketAddr, image::Handle>,
}

#[derive(Debug, Clone)]
pub enum ResultsListMessage {
    OpenPreview(SocketAddr),
}

impl ResultsList {
    pub fn push(&mut self, info: ServerInfo) {
        self.items.push(info);
    }
    pub fn clear(&mut self) {
        self.items.clear();
        self.avatars_small.clear();
        self.avatars_large.clear();
    }
    pub fn count(&self) -> usize { self.items.len() }
    pub fn items(&self) -> &[ServerInfo] { &self.items }

    pub fn get_by_addr(&self, addr: SocketAddr) -> Option<&ServerInfo> {
        self.items.iter().find(|s| s.addr == addr)
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

    pub fn refresh(&mut self, info: ServerInfo) -> Option<String> {
        let addr = info.addr;
        let new_favicon = info.favicon.clone();
        let favicon_changed;
        {
            let Some(s) = self.items.iter_mut().find(|s| s.addr == addr) else {
                return None;
            };
            s.online = info.online;
            s.max_players = info.max_players;
            s.latency_ms = info.latency_ms;
            s.samples = info.samples;
            s.sample_ids = info.sample_ids;
            s.mods = info.mods;
            favicon_changed = s.favicon != info.favicon;
            s.favicon = info.favicon;
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

        if favicon_changed {
            self.avatars_small.remove(&addr);
            self.avatars_large.remove(&addr);
            new_favicon
        } else {
            None
        }
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
        if !prefix.is_empty() && prefix.chars().next().map_or(false, |c| c.is_ascii_alphabetic()) {
            return (Some(prefix.to_string()), rest.to_string());
        }
    }
    (None, raw.to_string())
}

pub(crate) fn decode_favicon_avatars(favicon: &str) -> (Option<image::Handle>, Option<image::Handle>) {
    (
        favicon_handle(favicon, AvatarSize::SMALL),
        favicon_handle(favicon, AvatarSize::LARGE),
    )
}
