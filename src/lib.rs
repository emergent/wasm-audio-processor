use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_audio(input: &[u8]) -> Vec<u8> {
    log(input.len());
    input.to_vec()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(a: usize);
}
