use std::error::Error;

use serde_json::{json, Value};

pub fn fast_post(url: &str, data: Value) -> Result<Value, Box<dyn Error>> {
    let resp: Value = ureq::post(url)
        .set("User-Agent", "alertmsg")
        .set("Content-Type", "application/json")
        .send_json(json!(data))?
        .into_json()?;
    Ok(resp)
}
