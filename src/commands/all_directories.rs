use std::error::Error;
use cli_table::{Cell, print_stdout, Style, Table};
use colored::Colorize;
use meowith_connector::connector::connector::MeowithConnector;
use meowith_connector::dto::range::Range;
use crate::commands::handle_error;
use crate::format::data::{format_bytes, format_short_date_with_time};

pub async fn list_directories(
    connector: MeowithConnector,
    range: Option<Range>,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let response = connector.list_bucket_directories(range).await;

    let entities = handle_error(response).unwrap().entities;
    if verbose {
        let mut table = Vec::new();
        for entity in entities {
            table.push(vec![
                entity.name.bright_green().cell(),
                if entity.dir.is_some() {
                    entity.dir.unwrap().to_string()
                } else {
                    "root".to_string()
                }.cell(),
                format_bytes(entity.size).cell(),
                format_short_date_with_time(entity.created).cell(),
                format_short_date_with_time(entity.last_modified).cell(),
            ]);
        }
        let table = table
            .table()
            .title(vec![
                "Name".cell().bold(true),
                "ID".cell().bold(true),
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
