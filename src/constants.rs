pub mod plugin {
    pub const NAME: &str = "clipboard-pro";
    pub const FILE_DIR: &str = "clipboard-cache";
}

pub mod event {
    pub const CLIPBOARD_CHANGE: &str = "plugin:clipboard-pro://clipboard_change";
}

pub mod format {
    #[cfg(target_os = "macos")]
    pub const PNG_ALIASES: &[&str] = &["public.png"];

    #[cfg(not(target_os = "macos"))]
    pub const PNG_ALIASES: &[&str] = &["PNG", "image/png", "public.png"];
}
