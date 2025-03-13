use anyhow::Result;

pub mod init;

pub trait Command {
    fn execute(&self) -> Result<()>;
} 