use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TideStamp {
    timestamp: i32,
    utc_offset: i32,
    r#type: String,
    height: f32
}

pub fn find_high_and_low_tides(tides: &mut Vec<TideStamp>) {
    tides.sort_by(|a, b| a.height.total_cmp(&b.height));
    println!("{:?}", tides);
}

pub fn parse_tides(response_body: &String) -> Vec<TideStamp>{
    // Parse the string of data into serde_json::Value.
    let tides_array: Value = serde_json::from_str(response_body).unwrap_or(Value::Null);
    let mut tide_stamps: Vec<TideStamp> = vec!();

    for data in tides_array["data"]["tides"].as_array().into_iter() {
        for tide in data {
            let ts: TideStamp = serde_json::from_value(tide.to_owned()).unwrap();
            tide_stamps.push(ts);
        }
    };

    find_high_and_low_tides(&mut tide_stamps);
    return tide_stamps;
}

