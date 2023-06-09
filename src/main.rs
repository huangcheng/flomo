use clap::Parser;
use std::thread::{spawn, JoinHandle};

mod api;
mod cli;
mod config;

use api::send;
use cli::{Cli, Commands};
use config::{init_config, read_config_string, write_config, Config};

fn main() {
    let _cli = Cli::parse();
    let mut handle: Option<JoinHandle<()>> = None;

    match init_config() {
        true => (),
        false => println!("Failed to init config!"),
    }

    if let Some(memo) = _cli.memo {
        handle = Some(spawn(move || {
            match send(memo.as_str()) {
                Ok(link) => println!("{}", link),
                Err(err) => println!("{}", err),
            };
        }));
    }

    match &_cli.command {
        Some(Commands::Config { api, list }) => {
            match api {
                Some(api) => {
                    let config = Config {
                        api: Some(api.to_string()),
                    };

                    write_config(&config);
                }
                None => (),
            }

            match list {
                true => match read_config_string() {
                    Some(content) => println!("{}", content),
                    None => (),
                },
                false => (),
            }
        }
        _ => (),
    }

    if let Some(handle) = handle {
        handle.join().unwrap();
    }
}
