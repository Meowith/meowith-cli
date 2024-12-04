use crate::format::data::{format_bytes, format_short_date_with_time};
use cli_table::{Cell, CellStruct, Style};
use colored::Colorize;
use meowith_connector::dto::response::Entity;
use meowith_connector::error::{ConnectorError, ConnectorResponse, NodeClientError};

pub mod all_directories;
pub mod directory;
pub mod files;
pub mod stat_resource;

pub fn file_table_header() -> Vec<CellStruct> {
    vec![
        "Name".cell().bold(true),
        "Location".cell().bold(true),
        "Size".cell().bold(true),
        "Created".cell().bold(true),
        "Last modified".cell().bold(true),
    ]
}

pub fn map_file_cell(entity: Entity) -> Vec<CellStruct> {
    vec![
        entity.name.cell(),
        if entity.dir.is_some() {
            entity.dir.unwrap().to_string()
        } else {
            "/".to_string()
        }
        .cell(),
        format_bytes(entity.size).cell(),
        format_short_date_with_time(entity.created).cell(),
        format_short_date_with_time(entity.last_modified).cell(),
    ]
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
                        format!("Server returned error: {}", err.to_string())
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
