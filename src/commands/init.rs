use tracing::info;
use anyhow::Result;

use crate::commands::Command;

pub struct InitCommand;

impl Command for InitCommand {
    fn execute(&self) -> Result<()> {
        info!("Starting init command");
        println!("Hello World");
        info!("Completed init command");
        Ok(())
    }
} 