//! SVG icons embedded with `include_bytes!` and cached as handles.

use iced::widget::svg;
use once_cell::sync::Lazy;

macro_rules! embedded_icon {
    ($vis_fn:ident, $cache:ident, $file:literal) => {
        static $cache: Lazy<svg::Handle> = Lazy::new(|| {
            svg::Handle::from_memory(include_bytes!(concat!("../../../assets/", $file)).as_slice())
        });

        pub fn $vis_fn() -> svg::Handle {
            $cache.clone()
        }
    };
}

embedded_icon!(close, CLOSE, "close.svg");
embedded_icon!(copy, COPY, "copy.svg");
embedded_icon!(settings, SETTINGS, "settings.svg");
embedded_icon!(plus, PLUS, "plus.svg");
embedded_icon!(trash, TRASH, "trash.svg");
