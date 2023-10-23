//! Handle CRUD operations asynchronous in a file
//! - [x] Create

use tokio::fs::File;
// this trait added for using `write_all` method.
use tokio::io::AsyncWriteExt;
use tokio::io::Result;

pub(crate) async fn main() -> Result<()> {
    // create a file if not exist, else open the existing file
    let mut file = File::create("demo.txt").await?;
    file.write_all(b"Good morning!\n My name is Abhi and I joined Subspace Labs as Solidity Dev.")
        .await?;

    file.flush().await?;

    Ok(())
}
