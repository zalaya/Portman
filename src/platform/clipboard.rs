pub fn copy(text: impl Into<String>) -> Result<(), String> {
    arboard::Clipboard::new().and_then(|mut clipboard| clipboard.set_text(text.into())).map_err(|error| error.to_string())
}
