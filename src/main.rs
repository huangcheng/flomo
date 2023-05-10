use std::thread::{JoinHandle, spawn};
use clap::Parser;

mod cli;
mod config;
mod api;

use cli::{ Cli, Commands } ;
use config::{ Config, init_config, write_config, read_config_string };
use api::send;

fn main() {
    let _cli = Cli::parse();
    let mut handle: Option<JoinHandle<()>> = None;

    init_config();

    if let Some(memo) = _cli.memo {
        handle = Some(spawn(move || {
            let response = send(memo.as_str());

            if let Some(link) = response {
                println!("{}", link);
            }
        }));
    }

    match &_cli.command {
        Some(Commands::Config { api, list }) => {
            match api {
                Some(api) => {
                    let config = Config { api: Some(api.to_string()) };

                    write_config(&config);
                },
                None => (),
            }

            match list {
                true => {
                    match read_config_string() {
                        Some(content) => println!("{}", content),
                        None => (),
                    }
                }
                false => (),
            }
        },
        _ => (),
    }

    if let Some(handle) = handle {
        handle.join().unwrap();
    }
}
