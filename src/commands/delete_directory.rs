use std::error::Error;
use meowith_connector::connector::connector::MeowithConnector;
use crate::commands::handle_error;

pub async fn delete_directory(
    connector: MeowithConnector,
    path: String
) -> Result<(), Box<dyn Error>> {
    let response = connector.delete_directory(path.as_str()).await;

    handle_error(response).unwrap();
    Ok(())
}