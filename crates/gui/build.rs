fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        // Failing to embed resources shouldn't abort a dev build.
        let _ = res.compile();
    }
}
