use crate::types::app::AppInfo;
use serde::{Deserialize, Serialize};
use std::error::Error;
pub mod local;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(u8)]
pub enum ContentStoreType {
    Local,
}

/// A trait representing a content store responsible for managing
/// application-specific information and authentication tokens.
pub trait ContentStore {
    /// Sets the authentication token.
    ///
    /// # Arguments
    ///
    /// * `token` - A `String` representing the authentication token to be stored.
    ///
    /// # Example
    ///
    /// ```
    /// let token = "my_secure_token".to_string();
    /// ContentStore::set_token(token);
    /// ```
    fn set_token(&self, token: String) -> Result<(), Box<dyn Error>>;

    /// Sets the application information.
    ///
    /// # Arguments
    ///
    /// * `app_info` - An instance of `AppInfo` containing application metadata.
    ///
    /// # Example
    ///
    /// ```
    /// let app_info = AppInfo::new(app_id, bucket_id);
    /// ContentStore::set_app(app_info);
    /// ```
    fn set_app(&self, app_info: AppInfo) -> Result<(), Box<dyn Error>>;

    /// Retrieves the application information.
    ///
    /// # Returns
    ///
    /// * An instance of `AppInfo` representing the stored application metadata.
    ///
    /// # Example
    ///
    /// ```
    /// let app_info = ContentStore::get_app();
    /// println!("App Name: {}, Version: {}", app_info.app_id, app_info.bucket_id);
    /// ```
    fn get_app(&self) -> Result<Option<AppInfo>, Box<dyn Error>>;

    /// Retrieves the authentication token.
    ///
    /// # Returns
    ///
    /// * A `String` representing the stored authentication token.
    ///
    /// # Example
    ///
    /// ```
    /// let token = ContentStore::get_token();
    /// println!("Token: {}", token);
    /// ```
    fn get_token(&self) -> Result<Option<String>, Box<dyn Error>>;
}
