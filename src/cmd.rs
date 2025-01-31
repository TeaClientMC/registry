use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum SubCommands {
    Init {},
    Search {},
}
