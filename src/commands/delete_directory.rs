use crate::commands::handle_error;
use meowith_connector::connector::connector::MeowithConnector;
use std::error::Error;

pub async fn delete_directory(
    connector: MeowithConnector,
    path: String,
    recursive: bool,
) -> Result<(), Box<dyn Error>> {
    let response = connector.delete_directory(&path, recursive).await;

    handle_error(response).unwrap();
    Ok(())
}
