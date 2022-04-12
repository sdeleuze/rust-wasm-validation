use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    let window = web_sys::window().expect("no global `window` exists");
    window.alert_with_message("Hello Rust").unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
