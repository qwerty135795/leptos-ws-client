use leptos::tracing;
use serde::{Deserialize, Serialize};
use leptos::wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/static/socket.js")]
extern "C" {
    pub fn connectToSocketIO(url: &str);
    pub fn search_users(username: &str);
    // type Manager;
    // #[wasm_bindgen(constructor)]
    // fn new(url:&str, option:) -> Manager;

}
#[wasm_bindgen(js_namespace = console)]
extern "C" {
    pub fn log(arg: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
#[wasm_bindgen]
pub fn receive_users(data: &str) {
    let users:Vec<UserDTO> = serde_json::from_str(data).unwrap_or_else(|err| {
        console_log!("{}", err);
        Vec::new()
    });
    tracing::info!(?users);
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserDTO {
    pub id: i64,
    pub username:String
}