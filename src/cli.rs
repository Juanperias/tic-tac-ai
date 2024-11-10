use clap::Parser;
use clap_verbosity_flag::Verbosity;

use crate::commands::Commands;

#[derive(Parser)]
#[command(name = "Tic Tac ai")]
#[command(about = "A simple artificial intelligence model to demonstrate the qualities of rust")]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,

    #[clap(flatten)]
    verbose: Verbosity,
}

pub fn get() -> Cli {
    let cli = Cli::parse();
    let log_filter = cli.verbose.log_level_filter();

    env_logger::Builder::new()
        .default_format()
        .parse_default_env()
        .filter_level(log_filter)
        .init();

    cli
}
