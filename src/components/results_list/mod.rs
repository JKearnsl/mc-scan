mod avatar;
mod item;

use iced::widget::{column, container, text};
use iced::{Element, Fill, Padding, Theme};

use crate::components::ui::scrollbar;
use crate::i18n::Tr;
use crate::scanner::types::ServerInfo;
use crate::styles::{c, is_dark, SANS};

use item::server_card;

#[derive(Default)]
pub struct ResultsList {
    items: Vec<ServerInfo>,
}

#[derive(Debug, Clone)]
pub enum ResultsListMessage {}

impl ResultsList {
    pub fn push(&mut self, info: ServerInfo) { self.items.push(info); }
    pub fn clear(&mut self) { self.items.clear(); }
    pub fn count(&self) -> usize { self.items.len() }

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

        let mut col = column![].spacing(10).padding(Padding::from([12, 16]));
        for info in &self.items {
            col = col.push(server_card(info, tr));
        }

        scrollbar(col).into()
    }
}
