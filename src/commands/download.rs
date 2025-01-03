use crate::commands::handle_error;
use meowith_connector::connector::connector::MeowithConnector;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

pub async fn download_file(
    connector: MeowithConnector,
    path: String,
    output: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let response = connector.download_file(path.as_str()).await;
    let response = handle_error(response).unwrap();
    let output = output.unwrap_or(response.name);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(&output)
        .unwrap_or_else(|_| panic!("Could not write into file {}", output));
    let bytes = response.response.bytes().await?;
    file.write_all(&bytes)?;
    file.sync_all()?;

    Ok(())
}
