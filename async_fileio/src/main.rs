use tokio::io::Result;

mod create;
mod read;

#[tokio::main]
async fn main() -> Result<()> {
    create::main().await?;
    read::main().await?;

    Ok(())
}
