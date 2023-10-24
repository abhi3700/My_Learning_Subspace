#![allow(dead_code)]
#![allow(unused_imports)]
use tokio::io::Result;

mod create;
mod delete;
mod read;
mod update;

#[tokio::main]
async fn main() -> Result<()> {
    create::main().await?;
    read::main().await?;
    update::update_add_new_content().await?;
    update::update_replace_ewn_content().await?;

    Ok(())
}
