use std::error::Error;
use cli_table::{print_stdout, Style, Table};
use meowith_connector::connector::connector::MeowithConnector;
use meowith_connector::dto::range::Range;

use crate::commands::{file_table_header, handle_error, map_file_cell};

pub async fn list_files(
    connector: MeowithConnector,
    range: Option<Range>,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let response = connector.list_bucket_files(range).await;

    let entities = handle_error(response).unwrap().entities;
    if verbose {
        let mut table = Vec::new();
        for entity in entities {
            table.push(map_file_cell(entity));
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
