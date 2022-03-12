use std::error::Error;

fn main() {}

async fn get_daily_word() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?.text()?;
    Ok(())
}
