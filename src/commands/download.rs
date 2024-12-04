use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use meowith_connector::connector::connector::MeowithConnector;
use crate::commands::handle_error;

pub async fn download_file(
    connector: MeowithConnector,
    path: String,
    output: Option<String>
) -> Result<(), Box<dyn Error>> {
    let response = connector.download_file(path.as_str()).await;
    let response = handle_error(response).unwrap();
    let output = output.or(Some(response.name)).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&output).expect(format!("Could not write into file {}", output).as_str());
    let bytes = response.response.bytes().await?;
    file.write_all(&bytes)?;
    file.sync_all()?;

    Ok(())
}