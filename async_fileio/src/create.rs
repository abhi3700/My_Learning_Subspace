//! Create

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::Result;

pub(crate) async fn main() -> Result<()> {
    // create a file if not exist, else open the existing file
    let mut file = File::create("demo.txt").await?;
    file.write_all(b"Good morning!\n I am Abhi joined Subspace Labs as Solidity Dev.")
        .await?;

    file.flush().await?;

    Ok(())
}
