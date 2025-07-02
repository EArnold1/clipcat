use std::{thread, time::Duration};

use crate::{
    clipboard::board::read_clipboard,
    history::store::{get_last_item, save_item},
};

pub fn watcher() {
    loop {
        // Sleep for 5 seconds before polling the clipboard
        thread::sleep(Duration::from_secs(5));

        if let Some(value) = read_clipboard() {
            if let Ok(Some(item)) = get_last_item() {
                if item.value == value {
                    continue;
                } else if let Err(e) = save_item(&value) {
                    eprintln!("an error occurred: {:?}", e);
                    break;
                }
            }
        }
    }
}
