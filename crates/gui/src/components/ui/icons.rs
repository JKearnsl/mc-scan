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
embedded_icon!(export, EXPORT, "export.svg");
embedded_icon!(chevron_down, CHEVRON_DOWN, "chevron_down.svg");
embedded_icon!(chevron_up, CHEVRON_UP, "chevron_up.svg");
embedded_icon!(search, SEARCH, "search.svg");
embedded_icon!(filter, FILTER, "filter.svg");
embedded_icon!(sort, SORT, "sort.svg");
embedded_icon!(check, CHECK, "check.svg");
