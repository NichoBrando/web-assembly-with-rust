use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(message: &str);
}

#[wasm_bindgen]
pub fn do_sum(x: u32, y: u32) -> u32 {
    return x + y;
}

#[wasm_bindgen]
pub fn show_alert(message: &str) {
    alert(&format!("Your result is: {}", &message));
}