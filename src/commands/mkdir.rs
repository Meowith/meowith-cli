use crate::commands::handle_error;
use meowith_connector::connector::connector::MeowithConnector;
use std::error::Error;

pub async fn mkdir(connector: MeowithConnector, path: String) -> Result<(), Box<dyn Error>> {
    let response = connector.create_directory(path).await;

    handle_error(response).unwrap();
    Ok(())
}
