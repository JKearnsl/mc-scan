mod app;
mod components;
mod scanner;
mod styles;

use app::McScan;
use iced::{Font, Size, window};

const PLEX_SANS: &[u8]          = include_bytes!("../fonts/IBMPlexSans-Regular.ttf");
const PLEX_SANS_MEDIUM: &[u8]   = include_bytes!("../fonts/IBMPlexSans-Medium.ttf");
const PLEX_SANS_SEMIBOLD: &[u8] = include_bytes!("../fonts/IBMPlexSans-SemiBold.ttf");
const PLEX_MONO: &[u8]          = include_bytes!("../fonts/IBMPlexMono-Medium.ttf");
const PLEX_MONO_SEMIBOLD: &[u8] = include_bytes!("../fonts/IBMPlexMono-SemiBold.ttf");

fn main() -> iced::Result {
    iced::application(McScan::init, McScan::update, McScan::view)
        .title("mc-scan")
        .theme(McScan::theme)
        .subscription(McScan::subscription)
        .font(PLEX_SANS)
        .font(PLEX_SANS_MEDIUM)
        .font(PLEX_SANS_SEMIBOLD)
        .font(PLEX_MONO)
        .font(PLEX_MONO_SEMIBOLD)
        .default_font(Font::with_name("IBM Plex Sans"))
        .window(window::Settings {
            size: Size { width: 1060.0, height: 620.0 },
            min_size: Some(Size { width: 780.0, height: 480.0 }),
            resizable: true,
            ..window::Settings::default()
        })
        .run()
}
