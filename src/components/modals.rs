use iced::Element;

use crate::app::{McScan, Message};

pub fn settings(app: &McScan) -> Element<'_, Message> {
    crate::components::settings::render(app)
}

pub fn add_ranges(app: &McScan) -> Element<'_, Message> {
    crate::components::address_list::add_dialog::render(app)
}
