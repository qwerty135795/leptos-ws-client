use wasm_bindgen::prelude::*;
#[wasm_bindgen(module= "/static/socket.js")]
extern "C" {
    pub fn connectToSocketIO(url: &str);
}
