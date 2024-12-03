use crate::auth::ContentStore;
use crate::cli::config::ConfigProperties;
use crate::types::app::AppInfo;
use std::error::Error;

#[derive(Debug)]
pub struct LocalStore {
    pub config: ConfigProperties,
}

impl ContentStore for LocalStore {
    fn set_token(&self, token: String) -> Result<(), Box<dyn Error>> {
        let config = self.config.read()?;
        ConfigProperties {
            store_type: config.store_type.clone(),
            token: Some(token),
            app_info: config.app_info.clone(),
        }
        .save()
    }

    fn set_app(&self, app_info: AppInfo) -> Result<(), Box<dyn Error>> {
        let config = self.config.read()?;
        ConfigProperties {
            store_type: config.store_type.clone(),
            token: config.token.clone(),
            app_info: Some(app_info),
        }
        .save()
    }

    fn get_app(&self) -> Result<Option<AppInfo>, Box<dyn Error>> {
        self.config.read().map(|config| config.app_info)
    }

    fn get_token(&self) -> Result<Option<String>, Box<dyn Error>> {
        self.config.read().map(|config| config.token)
    }
}
