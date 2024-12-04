use crate::commands::handle_error;
use meowith_connector::connector::connector::MeowithConnector;
use std::error::Error;

pub async fn delete_file(connector: MeowithConnector, path: String) -> Result<(), Box<dyn Error>> {
    let response = connector.delete_file(path.as_str()).await;

    handle_error(response).unwrap();
    Ok(())
}
