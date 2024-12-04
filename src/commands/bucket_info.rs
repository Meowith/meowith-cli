use std::error::Error;

use cli_table::{print_stdout, Cell, Style, Table};
use meowith_connector::connector::connector::MeowithConnector;

use crate::commands::handle_error;
use crate::format::data::{format_bytes, format_short_date_with_time};
use crate::format::table::bucket_table_header;

pub async fn fetch_bucket_info(
    connector: MeowithConnector,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let response = connector.fetch_bucket_info().await;

    let entity = handle_error(response).unwrap();
    if verbose {
        let table = vec![vec![
            entity.name.cell(),
            entity.id.cell(),
            entity.app_id.cell(),
            entity.encrypted.cell(),
            format_bytes(entity.quota as u64).cell(),
            format_bytes(entity.space_taken as u64).cell(),
            format_short_date_with_time(entity.last_modified).cell(),
        ]]
        .table()
        .title(bucket_table_header())
        .bold(true);

        print_stdout(table)?;
    } else {
        println!(
            "{} {} {} {} {} {} {}",
            entity.name,
            entity.id,
            entity.app_id,
            entity.encrypted,
            format_bytes(entity.quota as u64),
            format_bytes(entity.space_taken as u64),
            format_short_date_with_time(entity.last_modified)
        );
    }
    Ok(())
}
