use std::env;
use std::error::Error;

use clap::Parser;
use cli::config::ConfigProperties;

use crate::auth::local::LocalStore;
use crate::auth::{ContentStore, ContentStoreType};
use crate::cli::args::{MeowithCommand, MeowithSubCommand};
use crate::cli::connector::connector;
use crate::cli::range_from_str;
use crate::commands::all_directories::list_directories;
use crate::commands::directory::list_directory_command;
use crate::commands::files::list_files;
use crate::types::app::AppInfo;

mod auth;
mod cli;
mod commands;
mod format;
mod types;

const MEOWITH_CONFIG: &str = ".meowith-config.yml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: MeowithCommand = MeowithCommand::parse();

    let global_config = ConfigProperties {
        store_type: ContentStoreType::Local,
        token: None,
        app_info: None,
    }
    .read()?;

    let content_store_type = global_config.store_type.clone();
    let content_store: Box<dyn ContentStore> = match content_store_type {
        ContentStoreType::Local => Box::new(LocalStore {
            config: global_config,
        }),
    };

    match args.command {
        MeowithSubCommand::Auth { token } => {
            content_store.set_token(token)?;
        }
        MeowithSubCommand::Use {
            app_id,
            bucket_id,
            node,
        } => {
            content_store.set_app(AppInfo {
                bucket_id,
                app_id,
                node,
            })?;
        }
        MeowithSubCommand::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        MeowithSubCommand::ListDirectories { range, verbose } => {
            let connector = connector(content_store)?;
            list_directories(
                connector,
                range.map(|range| range_from_str(range.as_str()).expect("Invalid range provided")),
                verbose,
            )
                .await?;
        }
        MeowithSubCommand::ListFiles { range, verbose } => {
            let connector = connector(content_store)?;
            list_files(
                connector,
                range.map(|range| range_from_str(range.as_str()).expect("Invalid range provided")),
                verbose,
            )
            .await?;
        }
        MeowithSubCommand::Directory {
            path,
            range,
            verbose,
        } => {
            let connector = connector(content_store)?;
            list_directory_command(
                connector,
                path,
                range.map(|range| range_from_str(range.as_str()).expect("Invalid range provided")),
                verbose,
            )
            .await?;
        }
    }

    Ok(())
}
