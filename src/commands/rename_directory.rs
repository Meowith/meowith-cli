use crate::commands::handle_error;
use meowith_connector::connector::connector::MeowithConnector;
use std::error::Error;

pub async fn rename_directory(
    connector: MeowithConnector,
    from: String,
    to: String,
) -> Result<(), Box<dyn Error>> {
    let response = connector.rename_directory(from.as_str(), to.as_str()).await;

    handle_error(response).unwrap();
    Ok(())
}
