// use wasm_bindgen::prelude::*;

// #[wasm_bindgen(start)]
// fn run () {
//   bare_bones();
//   using_a_macro();
//   using_web_sys();
// }

// #[wasm_bindgen]
// extern "C" {
//   #[wasm_bindgen(js_namespace = console)]
//   fn log(s: &str);

//   #[wasm_bindgen(js_namespace = console, js_name = log)]
//   fn log_u32(a: u32);

//   #[wasm_bindgen(js_namespace = console, js_name = log)]
//   fn log_many(a: &str, b: &str);
// }


// fn bare_bones() {
//   log("Hello from Rust!");
//   log_u32(42);
//   log_many("Logging", "many values!");
// }


// macro_rules! console_log {
//   // Note that this is using the `log` function imported above during
//   // `bare_bones`
//   ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }


// fn using_a_macro() {
//   console_log!("Hello {}!", "world");
//   console_log!("Let's print some numbers...");
//   console_log!("1 + 3 = {}", 1 + 3);
// }

// // And finally, we don't even have to define the `log` function ourselves! The
// // `web_sys` crate already has it defined for us.

// fn using_web_sys() {
//   use web_sys::console;

//   console::log_1(&"Hello using web-sys".into());

//   let js: JsValue = 4.into();
//   console::log_2(&"Logging arbitrary values looks like".into(), &js);
// }