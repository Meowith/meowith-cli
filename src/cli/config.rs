use crate::auth::ContentStoreType;
use crate::types::app::AppInfo;
use crate::MEOWITH_CONFIG;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigProperties {
    pub store_type: ContentStoreType,
    pub token: Option<String>,
    pub app_info: Option<AppInfo>,
}

impl ConfigProperties {
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let config = home_dir().unwrap().join(MEOWITH_CONFIG);
        let config = match fs::canonicalize(&config) {
            Ok(config) => Ok(config),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                File::create(&config)?;
                fs::canonicalize(config)
            }
            _ => Err(ErrorKind::Other.into()),
        }?;
            let mut file = OpenOptions::new().write(true).truncate(true).open(&config)?;
            let default_config_yaml = serde_yaml::to_string(&self)?;
            file.write_all(default_config_yaml.as_bytes())?;
            file.sync_all()?;

        Ok(())
    }

    pub fn read(default_config: Option<&ConfigProperties>) -> Result<Self, Box<dyn Error>> {
        let config = home_dir().unwrap().join(MEOWITH_CONFIG);
        let mut init = false;
        let config = match fs::canonicalize(&config) {
            Ok(config) => Ok(config),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                File::create(&config)?;
                init = true;
                fs::canonicalize(config)
            }
            _ => Err(ErrorKind::Other.into()),
        }?;
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&config)?;
        if init {
            let default_config = default_config
                .unwrap_or(&ConfigProperties {
                    store_type: ContentStoreType::Local,
                    token: None,
                    app_info: None,
                });
            let default_config_yaml = serde_yaml::to_string(default_config)?;
            file.write_all(default_config_yaml.as_bytes())?;
            file.sync_all()?;

            Ok(ConfigProperties {
                store_type: default_config.store_type.clone(),
                token: default_config.token.clone(),
                app_info: default_config.app_info.clone(),
            })
        } else {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            Ok(serde_yaml::from_slice::<ConfigProperties>(buf.as_slice())?)
        }
    }
}
