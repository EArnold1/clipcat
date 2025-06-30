use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    id: u8,
    pub value: String,
}

impl Item {
    fn new(value: String, len: u8) -> Self {
        Item { id: len + 1, value }
    }
}

pub fn load_history() -> std::io::Result<Vec<Item>> {
    let file = fs::read("history.json").ok();

    match file {
        Some(buf) => {
            let raw_json = String::from_utf8_lossy(&buf);
            let parsed: Vec<Item> = serde_json::from_str(&raw_json)?;
            Ok(parsed)
        }
        None => Ok(Vec::new()),
    }
}

pub fn save_history(history: &[Item]) -> std::io::Result<()> {
    let parsed: String = serde_json::to_string(history).unwrap();
    fs::write("history.json", parsed)?; // TODO: handle persistent storage properly

    Ok(())
}

pub fn save_item(value: &str) -> std::io::Result<()> {
    let mut history = load_history()?;

    let item = Item::new(value.into(), history.len() as u8);

    history.push(item);

    save_history(&history)?;

    Ok(())
}

pub fn get_item(id: u8) -> std::io::Result<Option<Item>> {
    let history = load_history()?;

    Ok(history.into_iter().find(|item| item.id == id))
}
