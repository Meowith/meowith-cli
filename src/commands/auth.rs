use std::error::Error;
use crate::auth::ContentStore;

pub fn set_token(content_store: &Box<dyn ContentStore>) -> Result<(), Box<dyn Error>> {
    println!("Provide your token");
    let token = rpassword::read_password()?;
    content_store.set_token(token)?;
    Ok(())
}
