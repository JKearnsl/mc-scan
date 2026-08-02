// Embeds the app icon and a default manifest into the Windows executable so the
// packaged .exe shows the icon in Explorer/taskbar. No-op on other platforms.
fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        // Failing to embed resources shouldn't abort a dev build.
        let _ = res.compile();
    }
}
