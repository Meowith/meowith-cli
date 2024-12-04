use crate::commands::handle_error;
use crate::format::data::{format_bytes, format_short_date_with_time};
use crate::format::table::{file_table_header, map_entity_cell};
use cli_table::{print_stdout, Style, Table};
use meowith_connector::connector::connector::MeowithConnector;
use std::error::Error;

pub async fn stat_resource(
    connector: MeowithConnector,
    path: String,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let response = connector.stat_resource(path).await;

    let entity = handle_error(response).unwrap();
    if verbose {
        let table = vec![map_entity_cell(entity)]
            .table()
            .title(file_table_header())
            .bold(true);

        print_stdout(table)?;
    } else {
        println!(
            "{} {} {} {} {}",
            entity.name,
            if entity.dir.is_some() {
                entity.dir.unwrap().to_string()
            } else {
                "/".to_string()
            },
            format_bytes(entity.size),
            format_short_date_with_time(entity.created),
            format_short_date_with_time(entity.last_modified)
        );
    }
    Ok(())
}
