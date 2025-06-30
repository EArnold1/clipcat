mod clipboard;
mod history;

use std::error::Error;

use clap::{Parser, Subcommand};

use crate::{
    clipboard::board::{read_clipboard, write_clipboard},
    history::store::{Item, get_item, save_item},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.cmd {
        Commands::Add { text } => save_item(&text)?,
        Commands::Save => {
            if let Some(value) = read_clipboard() {
                save_item(&value)?
            }
        }
        Commands::Copy { id } => {
            if let Some(Item { value, .. }) = get_item(id)? {
                write_clipboard(&value);
            }
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// add item to clipcat
    Add {
        /// value to be added
        text: String,
    },
    /// save current item on clipboard
    Save,
    /// copy value by id
    Copy {
        /// id of value to be copied
        id: u8,
    },
}
