// This is a basic cli to run the model and train it, you can run the train to train and export in a json, and with this you can run the web version.

mod cli;
mod commands;
mod models;

use anyhow::Result;

fn main() -> Result<()> {
    let cli = cli::get();

    cli.commands.exec()?;

    Ok(())
}
