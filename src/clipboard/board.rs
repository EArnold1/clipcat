use arboard::Clipboard;

pub fn read_clipboard() -> Option<String> {
    let mut clipboard = Clipboard::new().ok()?;
    clipboard.get_text().ok()
}

pub fn write_clipboard(text: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
}
