use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

extern crate ray_tracing;

use ray_tracing::vec3::Vec3;

#[wasm_bindgen]
pub fn greet(name: &str) {
    let text = format!("Hello, {}", name);
    alert(&text);
}

#[wasm_bindgen]
pub fn show() {
    let color = Vec3::fill(0.0);
    let text = format!("{:?}", color);
    alert(&text);
}
