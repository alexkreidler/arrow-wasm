use std::{io::BufReader, sync::Arc};

use arrow::csv::reader;
use wasm_bindgen::prelude::*;

use crate::{schema::Schema, utils::StringReader};

#[wasm_bindgen]
pub fn infer_schema_from_csv(
    content: String,
    // max_records: usize,
    // returns the schema as a jsvalue
) -> Result<JsValue, JsValue> {
    // let b = ",".as_bytes;
    // let r = unsafe { content.as_bytes_mut() };
    let r = StringReader::new(content.as_str());
    let mut b = BufReader::new(r);
    let out = reader::infer_reader_schema(
        b.get_mut(),
        b',',
        Some(50),
        true,
    );
    match out {
        Ok(schema) => {
            let sr = Arc::new(schema.0);
            return Ok(Schema::new(sr).to_json());
        }
        Err(e) => return Err(e.to_string().into()),
    }
}
