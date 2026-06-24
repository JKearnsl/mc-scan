mod avatar;
mod item;
pub mod preview_dialog;

use std::net::SocketAddr;

use iced::widget::{button, column, container, text};
use iced::{Background, Border, Element, Fill, Padding, Shadow, Theme};

use crate::components::ui::scrollbar;
use crate::i18n::Tr;
use crate::scanner::types::ServerInfo;
use crate::styles::{c, is_dark, SANS};

use item::server_card_content;

#[derive(Default)]
pub struct ResultsList {
    items: Vec<ServerInfo>,
}

#[derive(Debug, Clone)]
pub enum ResultsListMessage {
    OpenPreview(SocketAddr),
}

impl ResultsList {
    pub fn push(&mut self, info: ServerInfo) { self.items.push(info); }
    pub fn clear(&mut self) { self.items.clear(); }
    pub fn count(&self) -> usize { self.items.len() }
    pub fn items(&self) -> &[ServerInfo] { &self.items }

    pub fn get_by_addr(&self, addr: SocketAddr) -> Option<&ServerInfo> {
        self.items.iter().find(|s| s.addr == addr)
    }

    pub fn refresh(&mut self, info: ServerInfo) {
        if let Some(s) = self.items.iter_mut().find(|s| s.addr == info.addr) {
            s.online = info.online;
            s.max_players = info.max_players;
            s.latency_ms = info.latency_ms;
            s.samples = info.samples;
            s.ping_history.push(info.latency_ms);
            if s.ping_history.len() > 30 {
                s.ping_history.remove(0);
            }
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

        let mut col = column![].spacing(9).padding(Padding::from([12, 16]));
        for info in &self.items {
            let addr = info.addr;
            let content = server_card_content(info, tr);
            col = col.push(
                button(content)
                    .on_press(ResultsListMessage::OpenPreview(addr))
                    .style(card_btn_style)
                    .padding(Padding::from([13, 15]))
                    .width(Fill),
            );
        }

        scrollbar(col).into()
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
