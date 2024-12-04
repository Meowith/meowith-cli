use crate::format::table::{file_table_header, map_entity_cell};
use cli_table::{print_stdout, Style, Table};
use colored::Colorize;
use meowith_connector::dto::response::Entity;
use meowith_connector::error::{ConnectorError, ConnectorResponse, NodeClientError};
use std::error::Error;

pub mod all_directories;
pub mod bucket_info;
pub mod delete_directory;
pub mod delete_file;
pub mod directory;
pub mod download;
pub mod files;
pub mod mkdir;
pub mod rename_directory;
pub mod rename_file;
pub mod stat_resource;
pub mod upload;

pub fn display_formatted_entities(
    entities: Vec<Entity>,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    if verbose {
        let mut table = Vec::new();
        for entity in entities {
            table.push(map_entity_cell(entity));
        }
        let table = table.table().title(file_table_header()).bold(true);

        print_stdout(table)?;
    } else {
        for entity in entities {
            println!("{}", entity.name);
        }
    }
    Ok(())
}

pub fn handle_error<T>(response: ConnectorResponse<T>) -> ConnectorResponse<T> {
    if response.is_err() {
        let error = response.err().unwrap();
        match error {
            ConnectorError::Remote(err) => {
                let msg = match err {
                    NodeClientError::NotFound => "Resource was not found".to_string(),
                    NodeClientError::BadAuth => "Invalid token".to_string(),
                    _ => {
                        format!("Server returned error: {}", err)
                    }
                }
                .bright_red();
                panic!("{}", msg);
            }
            _ => panic!("{:?}", error),
        }
    }
    response
}
