//! Create

use tokio::io::Result;

mod create;

#[tokio::main]
async fn main() -> Result<()> {
    create::main().await?;

    Ok(())
}
