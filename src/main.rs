mod clipboard;
mod history;

use std::error::Error;

use clap::{Parser, Subcommand};

use crate::{
    clipboard::board::{read_clipboard, write_clipboard},
    history::store::{Item, clear_history, get_item, list_items, save_item},
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
            if let Some(Item { value, .. }) = get_item(&id)? {
                write_clipboard(&value);
            }
        }
        Commands::List => list_items()?,
        Commands::Clear => clear_history()?,
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
        id: String,
    },
    /// list all saved values
    List,
    /// clear history
    Clear,
}
