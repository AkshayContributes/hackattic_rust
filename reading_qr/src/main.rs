use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Problem {
    image_url: String,
}

#[derive(Serialize)]
struct Solution {
    code: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body: String = ureq::get("https://hackattic.com/challenges/reading_qr/problem?access_token=8763c525b9c22be6")
        .call()?
        .body_mut()
        .read_to_string()?;
    
    let problem: Problem = serde_json::from_str(&body)?;
    println!("Image URL: {}", problem.image_url);
    
    let bytes: Vec<u8> = ureq::get(&problem.image_url)
        .call()?
        .body_mut()
        .read_to_vec()?;
    
    println!("downloaded {} bytes", bytes.len());

    let img = image::load_from_memory(&bytes)?;

    let mut prep = rqrr::PreparedImage::prepare(img.to_luma8());
    let grids = prep.detect_grids();
    for grid in grids {
        let (_meta, content) = grid.decode()?;
        println!("QR Says: {}", content);
        let send_body = Solution {code: content.clone()};
        let response = ureq::post("https://hackattic.com/challenges/reading_qr/solve?access_token=8763c525b9c22be6")
        .send_json(&send_body)?
        .body_mut()
        .read_json::<serde_json::Value>()?;
        println!("Response: {}", response);
    }
    Ok(())
}
