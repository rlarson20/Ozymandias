use clap::Subcommand;
use anyhow::Result;

use crate::commands::Command;

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new knowledge base
    Init,
}

impl Commands {
    pub fn execute(&self) -> Result<()> {
        match self {
            Commands::Init => {
                let cmd = crate::commands::init::InitCommand;
                cmd.execute()
            }
        }
    }
} 