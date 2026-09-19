use sha2::{Sha256, Digest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body: String = ureq::get("https://hackattic.com/challenges/mini_miner/problem?access_token=8763c525b9c22be6")
        .call()?
        .body_mut()
        .read_to_string()?;
    
    let problem: serde_json::Value = serde_json::from_str(&body)?;
    let difficulty = problem["difficulty"].as_u64().unwrap();
    let mut block = problem["block"].clone();
    let mut nonce : u32 = 0;
    let winning_nonce: u32 = loop {
        block["nonce"] = serde_json::json!(nonce);
        let text = serde_json::to_string(&block)?;
        let hash = Sha256::digest(text.as_bytes());
        if  u64::from(leading_zero_bits(&hash)) >= difficulty {
            break nonce;
        }
        nonce += 1;
    };

    println!("Winning nonce: {} for difficulty {}", winning_nonce, difficulty);

    let send_body = serde_json::json!({"nonce": winning_nonce});
    let response = ureq::post("https://hackattic.com/challenges/mini_miner/solve?access_token=8763c525b9c22be6")
        .send_json(&send_body)?
        .body_mut()
        .read_json::<serde_json::Value>()?;

    println!("Response: {}", response);
    Ok(())
}

fn leading_zero_bits(bytes: &[u8]) -> u32 {
    let mut count : u32 = 0;
    for &b in bytes {
        if b == 0 {
            count += 8;
        } else {
            count += b.leading_zeros();
            break;
        }
    }
    return count;
}


