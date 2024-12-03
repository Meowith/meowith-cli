use crate::auth::ContentStore;
use meowith_connector::connector::connector::MeowithConnector;
use std::error::Error;

pub fn connector(store: Box<dyn ContentStore>) -> Result<MeowithConnector, Box<dyn Error>> {
    let token = store
        .get_token()?
        .expect("No token provided. Use `mcli auth <token>` to set token");
    let app_info = store
        .get_app()?
        .expect("No AppInfo provided. Use `mcli use <app-id> <bucket-id>`");

    Ok(MeowithConnector::new(
        token.as_str(),
        app_info.bucket_id,
        app_info.app_id,
        app_info.node,
    ))
}
