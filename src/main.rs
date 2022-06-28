use crate::utils::config::Config;

extern crate tokio;

pub mod tests;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let config: Config = utils::config::Config::new();
    println!("{:#?}", config);
    Ok(())
}
