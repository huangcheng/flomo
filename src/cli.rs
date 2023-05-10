use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Save a memo.
    pub memo: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get and set options
    Config {
        /// Specify the api to save memos.
        #[arg(long)]
        api: Option<String>,
        /// List all variables set in config file, along with their values.
        #[arg(short, long)]
        list: bool,
    },
}
