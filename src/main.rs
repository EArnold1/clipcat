mod clipboard;
mod history;

use std::error::Error;

use clap::{Parser, Subcommand};

use crate::history::store::save_item;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.cmd {
        Commands::Add { text } => save_item(&text)?,
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
}
