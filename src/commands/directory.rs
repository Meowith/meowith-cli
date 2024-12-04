use crate::commands::handle_error;
use crate::format::data::{format_bytes, format_short_date_with_time};
use cli_table::{print_stdout, Cell, Style, Table};
use colored::Colorize;
use meowith_connector::connector::connector::MeowithConnector;
use meowith_connector::dto::range::Range;
use std::error::Error;

pub async fn list_directory_command(
    connector: MeowithConnector,
    path: String,
    range: Option<Range>,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let response = connector.list_directory(path, range).await;

    let entities = handle_error(response).unwrap().entities;
    if verbose {
        let mut table = Vec::new();
        for entity in entities {
            table.push(vec![
                if entity.is_dir {
                    format!("{} ({})", entity.name, entity.dir_id.unwrap()).bright_green()
                } else {
                    entity.name.white()
                }
                .cell(),
                format_bytes(entity.size).cell(),
                format_short_date_with_time(entity.created).cell(),
                format_short_date_with_time(entity.last_modified).cell(),
            ]);
        }
        let table = table
            .table()
            .title(vec![
                "Name".cell().bold(true),
                "Size".cell().bold(true),
                "Created".cell().bold(true),
                "Last modified".cell().bold(true),
            ])
            .bold(true);

        print_stdout(table)?;
    } else {
        for entity in entities {
            println!("{}", entity.name);
        }
    }
    Ok(())
}
