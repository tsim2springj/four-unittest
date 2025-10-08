use colored::*;
use std::error::Error;

pub async fn run(action: &str) -> Result<(), Box<dyn Error>> {
    match action {
        "generate" => println!("🪙 Generating new wallet... (mock)"),
        "check" => println!("📊 Checking balances... (mock)"),
        "wrap" => println!("🔁 Wrapping BNB to WBNB... (mock)"),
        "unwrap" => println!("🔄 Unwrapping WBNB to BNB... (mock)"),
        _ => println!("❓ Unknown wallet action."),
    }
    Ok(())
}
