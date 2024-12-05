extern crate core;

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
use crate::commands::auth::set_token;
use crate::commands::bucket_info::fetch_bucket_info;
use crate::commands::delete_directory::delete_directory;
use crate::commands::delete_file::delete_file;
use crate::commands::directory::list_directory_command;
use crate::commands::download::download_file;
use crate::commands::files::list_files;
use crate::commands::mkdir::mkdir;
use crate::commands::rename_directory::rename_directory;
use crate::commands::rename_file::rename_file;
use crate::commands::stat_resource::stat_resource;
use crate::commands::upload::upload_file;
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

    let global_config = ConfigProperties::read(None)?;

    let content_store_type = global_config.store_type.clone();
    let content_store: Box<dyn ContentStore> = match content_store_type {
        ContentStoreType::Local => Box::new(LocalStore {
            config: global_config,
        }),
    };

    match args.command {
        MeowithSubCommand::Auth => {
            set_token(&content_store)?;
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
        MeowithSubCommand::StatResource { path, verbose } => {
            let connector = connector(content_store)?;
            stat_resource(connector, path, verbose).await?;
        }
        MeowithSubCommand::BucketInfo { verbose } => {
            let connector = connector(content_store)?;
            fetch_bucket_info(connector, verbose).await?;
        }
        MeowithSubCommand::Mkdir { path } => {
            let connector = connector(content_store)?;
            mkdir(connector, path).await?;
        }
        MeowithSubCommand::RenameDirectory { from, to } => {
            let connector = connector(content_store)?;
            rename_directory(connector, from, to).await?;
        }
        MeowithSubCommand::DeleteDirectory { path } => {
            let connector = connector(content_store)?;
            delete_directory(connector, path).await?;
        }
        MeowithSubCommand::Download { path, output } => {
            let connector = connector(content_store)?;
            download_file(connector, path, output).await?;
        }
        MeowithSubCommand::RenameFile { from, to } => {
            let connector = connector(content_store)?;
            rename_file(connector, from, to).await?;
        }
        MeowithSubCommand::Upload {
            local_path,
            remote_path,
        } => {
            let connector = connector(content_store)?;
            upload_file(connector, local_path, remote_path).await?;
        }
        MeowithSubCommand::DeleteFile { path } => {
            let connector = connector(content_store)?;
            delete_file(connector, path).await?;
        }
    }

    Ok(())
}
