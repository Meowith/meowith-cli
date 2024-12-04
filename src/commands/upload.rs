use crate::commands::handle_error;
use meowith_connector::connector::connector::MeowithConnector;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

pub async fn upload_file(
    connector: MeowithConnector,
    local_path: String,
    remote_path: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let file_name = Path::new(local_path.as_str())
        .file_name()
        .expect("Local file not found")
        .to_str()
        .unwrap();
    let mut file = OpenOptions::new().read(true).open(&local_path)?;
    let file_metadata = file
        .metadata()
        .expect("Error accessing local file metadata");
    let remote_path = remote_path.unwrap_or(format!("/{}", file_name));
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let response = connector
        .upload_oneshot(
            reqwest::Body::from(bytes),
            remote_path.as_str(),
            file_metadata.len(),
        )
        .await;

    handle_error(response).unwrap();
    Ok(())
}
