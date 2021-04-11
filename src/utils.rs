#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[macro_export]
macro_rules! impl_to_json {
    ($struct_name:ident) => {
        #[wasm_bindgen]
        impl $struct_name {
            /// Generate a JSON representation.
            #[wasm_bindgen(js_name = toJSON)]
            pub fn to_json(&self) -> JsValue {
                JsValue::from_serde(&self.0.to_json()).unwrap()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_to_string {
    ($struct_name:ident) => {
        #[wasm_bindgen]
        impl $struct_name {
            /// Generate a String representation.
            #[wasm_bindgen(js_name = toString)]
            pub fn to_string(&self) -> String {
                format!("{:?}", self.0)
            }
        }
    };
}

use std::io::Read;
use std::io::Result;
use std::slice::Iter;

pub struct StringReader<'a> {
    iter: Iter<'a, u8>,
}

impl<'a> StringReader<'a> {
    /// Wrap a string in a `StringReader`, which implements `std::io::Read`.
    pub fn new(data: &'a str) -> Self {
        Self {
            iter: data.as_bytes().iter(),
        }
    }
}

impl<'a> Read for StringReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        for i in 0..buf.len() {
            if let Some(x) = self.iter.next() {
                buf[i] = *x;
            } else {
                return Ok(i);
            }
        }
        Ok(buf.len())
    }
}
