use wasm_bindgen::prelude::*;

pub struct Card {
    pub text: String,
}

impl Card {
    pub fn new(text: String) -> Self {
        Self {
            text: text,
        }
    }
}