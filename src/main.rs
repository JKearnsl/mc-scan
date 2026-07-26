mod app;
mod components;
mod i18n;
mod scanner;
mod styles;

use app::McScan;
use iced::{Font, Size, window};

const PLEX_SANS: &[u8] = include_bytes!("../assets/fonts/IBMPlexSans-Regular.ttf");
const PLEX_SANS_MEDIUM: &[u8] = include_bytes!("../assets/fonts/IBMPlexSans-Medium.ttf");
const PLEX_SANS_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/IBMPlexSans-SemiBold.ttf");
const PLEX_MONO: &[u8] = include_bytes!("../assets/fonts/IBMPlexMono-Medium.ttf");
const PLEX_MONO_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/IBMPlexMono-SemiBold.ttf");

const APP_ICON: &[u8] = include_bytes!("../assets/icon.png");

#[cfg(unix)]
fn raise_fd_limit() {
    unsafe {
        let mut lim = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
        if libc::getrlimit(libc::RLIMIT_NOFILE, &mut lim) == 0 && lim.rlim_cur < lim.rlim_max {
            lim.rlim_cur = lim.rlim_max;
            let _ = libc::setrlimit(libc::RLIMIT_NOFILE, &lim);
        }
    }
}

fn main() -> iced::Result {
    #[cfg(unix)]
    raise_fd_limit();

    iced::application(McScan::init, McScan::update, McScan::view)
        .title(env!("CARGO_PKG_NAME"))
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
            icon: window::icon::from_file_data(APP_ICON, None).ok(),
            platform_specific: window::settings::PlatformSpecific {
                // Must match the basename of the .desktop file and its
                // StartupWMClass so Wayland/X11 links the window to its icon.
                application_id: String::from("mc-scan"),
                ..Default::default()
            },
            ..window::Settings::default()
        })
        .run()
}
