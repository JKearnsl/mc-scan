mod app;
mod components;
mod config;
mod export;
mod i18n;
mod styles;

use app::McScan;
use iced::{Font, Size, window};

/// Product name shown in the window title and header. Kept separate from the
/// crate name (`gui`) so renaming the crate can't change what the user sees.
pub const APP_NAME: &str = "mc-scan";

const PLEX_SANS: &[u8] = include_bytes!("../assets/fonts/IBMPlexSans-Regular.ttf");
const PLEX_SANS_MEDIUM: &[u8] = include_bytes!("../assets/fonts/IBMPlexSans-Medium.ttf");
const PLEX_SANS_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/IBMPlexSans-SemiBold.ttf");
const PLEX_MONO: &[u8] = include_bytes!("../assets/fonts/IBMPlexMono-Medium.ttf");
const PLEX_MONO_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/IBMPlexMono-SemiBold.ttf");

const APP_ICON: &[u8] = include_bytes!("../assets/icon.png");

// macOS reports the hard limit as "infinity", which `setrlimit` rejects (the
// real ceiling is `kern.maxfilesperproc`), so raising the soft limit to the hard
// limit fails there and leaves it at the default 256. Fall back through concrete
// targets until one is accepted.
#[cfg(unix)]
fn raise_fd_limit() {
    unsafe {
        let mut lim = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        if libc::getrlimit(libc::RLIMIT_NOFILE, &mut lim) != 0 || lim.rlim_cur >= lim.rlim_max {
            return;
        }
        let original_cur = lim.rlim_cur; // compared against, since lim.rlim_cur gets overwritten below
        for &target in &[lim.rlim_max, 1_048_576, 65_536, 10_240] {
            if target <= original_cur {
                continue;
            }
            lim.rlim_cur = target;
            if libc::setrlimit(libc::RLIMIT_NOFILE, &lim) == 0 {
                break;
            }
        }
    }
}

fn main() -> iced::Result {
    init_tracing();

    #[cfg(unix)]
    raise_fd_limit();

    iced::application(McScan::init, McScan::update, McScan::view)
        .title(APP_NAME)
        .theme(McScan::theme)
        .subscription(McScan::subscription)
        .font(PLEX_SANS)
        .font(PLEX_SANS_MEDIUM)
        .font(PLEX_SANS_SEMIBOLD)
        .font(PLEX_MONO)
        .font(PLEX_MONO_SEMIBOLD)
        .default_font(Font::with_name("IBM Plex Sans"))
        .window(window::Settings {
            size: Size {
                width: 1060.0,
                height: 620.0,
            },
            min_size: Some(Size {
                width: 780.0,
                height: 480.0,
            }),
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

/// Logs to stderr, filtered by `RUST_LOG`. The default shows our scan summaries
/// and errors from other crates, but keeps per-probe detail off (it would be one
/// line per host on a wide scan) and silences noisy backend warnings (wgpu/gles);
/// raise it with e.g. `RUST_LOG=mc_scan=debug` to see probe outcomes.
fn init_tracing() {
    use tracing_subscriber::EnvFilter;
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("error,mc_scan=info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}
