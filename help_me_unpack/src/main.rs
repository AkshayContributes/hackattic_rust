use serde::{Serialize, Deserialize};
use base64::prelude::*;


#[derive(Deserialize)]
struct Problem {
    bytes: String,
}

#[derive(Serialize)]
struct Solution {
    int: i32,
    uint: u32,
    short: i16,
    float: f32,
    double: f64,
    big_endian_double: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body: String = ureq::get("https://hackattic.com/challenges/help_me_unpack/problem?access_token=8763c525b9c22be6")
    .call()?
    .body_mut()
    .read_to_string()?;

    let problem: Problem = serde_json::from_str(&body)?;
    let raw : Vec<u8> = BASE64_STANDARD.decode(&problem.bytes)?;
    let int = i32::from_le_bytes(raw[0..4].try_into()?);
    let uint = u32::from_le_bytes(raw[4..8].try_into()?);
    let short = i16::from_le_bytes(raw[8..10].try_into()?);
    let float = f32::from_le_bytes(raw[12..16].try_into()?);
    let double = f64::from_le_bytes(raw[16..24].try_into()?);
    let big_endian_double = f64::from_be_bytes(raw[24..32].try_into()?);

    let solution = Solution {
        int,
        uint,
        short,
        float,
        double,
        big_endian_double,
    };

    let response =ureq::post("https://hackattic.com/challenges/help_me_unpack/solve?access_token=8763c525b9c22be6")
        .send_json(&solution)?
        .body_mut()
        .read_json::<serde_json::Value>()?;
    
    println!("Response: {}", response);
    Ok(())
}

