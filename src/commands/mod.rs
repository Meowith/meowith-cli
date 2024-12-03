use colored::Colorize;
use meowith_connector::error::{ConnectorError, ConnectorResponse, NodeClientError};

pub mod directory;
pub mod files;
pub mod all_directories;

pub fn handle_error<T>(response: ConnectorResponse<T>) -> ConnectorResponse<T> {
    if response.is_err() {
        let error = response.err().unwrap();
        match error {
            ConnectorError::Remote(err) => {
                let msg = match err {
                    NodeClientError::NotFound => {
                        "Resource was not found".to_string()
                    }
                    NodeClientError::BadAuth => {
                        "Invalid token".to_string()
                    }
                    _ => {
                        format!("Server returned error: {}", err.to_string())
                    }
                }.bright_red();
                panic!("{}", msg);
            }
            _ => panic!("{:?}", error),
        }
    }
    response
}