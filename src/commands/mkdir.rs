use std::error::Error;
use meowith_connector::connector::connector::MeowithConnector;
use crate::commands::handle_error;

pub async fn mkdir(
    connector: MeowithConnector,
    path: String
) -> Result<(), Box<dyn Error>> {
    let response = connector.create_directory(path).await;

    handle_error(response).unwrap();
    Ok(())
}