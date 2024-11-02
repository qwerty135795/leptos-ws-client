use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RegisterModel {
    pub username:String,
    pub email:String,
    pub password:String
}
impl Default for RegisterModel {
    fn default() -> Self {
        Self {
            username:String::new(),
            email:String::new(),
            password:String::new()
        }
    }
}
#[derive(Debug, Serialize)]
pub struct LoginModel {
    pub username:String,
    pub password:String
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tokens {
    pub access_token:String,
    pub token_type:String
}