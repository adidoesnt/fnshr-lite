use std::env;
use dotenv::dotenv;

pub fn get_env_var(name: &str) -> String {
    dotenv().ok();
    let err_msg: String = format!("Environment variable `{}` not found", name);
    env::var(name).expect(&err_msg)
}