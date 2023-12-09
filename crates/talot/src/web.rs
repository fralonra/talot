use wasm_bindgen::prelude::*;

use crate::run;

#[wasm_bindgen]
pub fn run_web() {
    run();
}
