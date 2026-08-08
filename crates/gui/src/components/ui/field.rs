use iced::widget::text::IntoFragment;
use iced::widget::{Space, column, container};
use iced::{Element, Fill, Padding};

use super::typography::caption;

/// Caption above arbitrary content. Accepts both `&str` and `String` labels.
pub fn field<'a, M: 'a>(label: impl IntoFragment<'a>, content: Element<'a, M>) -> Element<'a, M> {
    container(column![caption(label, 10), Space::new().height(8), content].width(Fill))
        .padding(Padding::from([10, 22]))
        .into()
}
