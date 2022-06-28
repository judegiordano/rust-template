use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub rust_backtrace: u32,
}

pub fn parse_env_variable<T: FromStr>(name: &str) -> T {
    // get variable
    let normalized: String = name.trim().to_uppercase().trim().to_string();
    let value: String = env::var(&normalized).unwrap_or_else(|_| panic!("{} not set", &normalized));
    // parse data type
    value
        .parse::<T>()
        .unwrap_or_else(|_| panic!("error parsing environment variable {}", &normalized))
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();
        Config {
            rust_backtrace: parse_env_variable::<u32>("RUST_BACKTRACE"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
