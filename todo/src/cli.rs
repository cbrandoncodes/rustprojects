use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI todo app")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(long, default_value = "todos.json")]
    pub file: String,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add { text: String },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
}
