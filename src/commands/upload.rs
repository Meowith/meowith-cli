use crate::commands::handle_error;
use meowith_connector::connector::connector::MeowithConnector;
use std::error::Error;
use std::path::Path;
use tokio::fs::OpenOptions;

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
    let file = OpenOptions::new().read(true).open(&local_path).await?;
    let file_metadata = file
        .metadata()
        .await
        .expect("Error accessing local file metadata");
    let remote_path = remote_path.unwrap_or(format!("/{}", file_name));
    let response = connector
        .upload_oneshot(
            reqwest::Body::from(file),
            remote_path.as_str(),
            file_metadata.len(),
        )
        .await;

    handle_error(response).unwrap();
    Ok(())
}
