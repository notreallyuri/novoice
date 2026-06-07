#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("Note: .env not found or unreadable: {e}")
    }

    api::start().await?;

    Ok(())
}
