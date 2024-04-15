use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use jsonwebtoken::{encode, decode, Header, Validation, Algorithm, EncodingKey, DecodingKey};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub struct Config {
    pub secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            secret: std::env::var("SECRET").unwrap(),
        }
    }
}

pub fn get_token_for_user(username: &str, config: &Config) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let payload = LoginResponse {
        token: username.to_string(),
    };
    let token = encode(&Header::default(), &payload, &EncodingKey::from_secret(config.secret.as_ref())).unwrap();
    println!("Token: {:?}", token);
    return token;
}

pub mod tests {
    use super::*;
    #[test]
    fn test1(){
        get_token_for_user("Tommy", &Config::from_env());
    }
}
