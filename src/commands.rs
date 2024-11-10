mod run;
mod train;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Train {
        #[clap(short, long)]
        file: String,

        #[clap(short, long)]
        dist: String,
    },
    Run {
        game: String,

        #[clap(short, long)]
        file: String,
    },
}

impl Commands {
    pub fn exec(&self) -> Result<()> {
        match self {
            Self::Train { file, dist } => {
                train::start(file, dist)?;
            }
            Self::Run { file, game } => {
                run::model(game, file)?;
            }
        }

        Ok(())
    }
}
