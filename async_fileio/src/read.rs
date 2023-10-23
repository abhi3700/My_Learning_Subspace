//! Handle CRUD operations asynchronous in a file
//! - [x] Read
//!
//! Here, we can open the file using either of these methods:
//! - `File::open()`
//! - `OpenOptions::new()`
//!
//! We can also read the file:
//! - entirely via `read_to_end`
//! - partially via `read`

#[allow(unused_imports)]
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncReadExt;
use tokio::io::Result;

pub(crate) async fn main() -> Result<()> {
    // opening a file in read-only mode.
    // let mut file = File::open("demo.txt").await?;    // M-1
    let mut file = OpenOptions::new().read(true).open("demo.txt").await?; // M-2

    // read the whole file
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    // read partially to 10 bytes
    let mut buffer = [0; 10];
    file.read(&mut buffer).await?;

    Ok(())
}
