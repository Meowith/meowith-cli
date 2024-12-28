use crate::format::data::{format_bytes, format_short_date_with_time};
use cli_table::{Cell, CellStruct, Style};
use colored::Colorize;
use meowith_connector::dto::response::Entity;

pub fn map_entity_cell(entity: Entity) -> Vec<CellStruct> {
    vec![
        if entity.is_dir { "dir" } else { "file" }.cell(),
        if entity.is_dir {
            format!("{} ({})", entity.name, entity.dir_id.unwrap()).bright_green()
        } else {
            entity.name.white()
        }
        .cell(),
        if entity.dir.is_some() && entity.dir.unwrap().to_u128_le() != 0 {
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

pub fn file_table_header() -> Vec<CellStruct> {
    vec![
        "Type".cell().bold(true),
        "Name".cell().bold(true),
        "Location".cell().bold(true),
        "Size".cell().bold(true),
        "Created".cell().bold(true),
        "Last modified".cell().bold(true),
    ]
}

pub fn entity_table_header() -> Vec<CellStruct> {
    vec![
        "Name".cell().bold(true),
        "ID".cell().bold(true),
        "Size".cell().bold(true),
        "Created".cell().bold(true),
        "Last modified".cell().bold(true),
    ]
}

pub fn bucket_table_header() -> Vec<CellStruct> {
    vec![
        "Name".cell().bold(true),
        "ID".cell().bold(true),
        "App ID".cell().bold(true),
        "Encryption".cell().bold(true),
        "Quota".cell().bold(true),
        "Used".cell().bold(true),
        "Last modified".cell().bold(true),
    ]
}
