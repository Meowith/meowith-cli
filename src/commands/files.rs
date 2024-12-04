use meowith_connector::connector::connector::MeowithConnector;
use meowith_connector::dto::range::Range;
use std::error::Error;

use crate::commands::{display_formatted_entities, handle_error};

pub async fn list_files(
    connector: MeowithConnector,
    range: Option<Range>,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let response = connector.list_bucket_files(range).await;

    let entities = handle_error(response).unwrap().entities;
    display_formatted_entities(entities, verbose)
}
