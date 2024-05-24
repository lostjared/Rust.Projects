
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello_world() -> String {
    "hello, world".to_string()
}

#[wasm_bindgen]
struct Object {
    x: i32,
}

#[wasm_bindgen]
impl Object  {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Object {
            x: 0
        }

    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i32 {
        self.x
    }
    #[wasm_bindgen(setter)]
    pub fn x(&self, value: i32) {
        self.x = value;
    }

}