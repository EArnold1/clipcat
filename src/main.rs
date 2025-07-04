mod clipboard;
mod history;
mod services;

use std::error::Error;

use clap::{Parser, Subcommand};

use clipboard::board::{read_clipboard, write_clipboard};
use history::store::{Item, clear_history, get_item, list_items, save_item, search};
use services::background::watcher;

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
        Commands::Search { query } => search(&query)?,
        Commands::Background => watcher(),
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
    /// search for an item by id or content
    Search {
        /// query to search for
        query: String,
    },
    /// clear history
    Clear,
    /// start background watcher
    Background,
}
