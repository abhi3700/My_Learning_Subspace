//! Handle CRUD operations asynchronous in a file
//! - [x] Read
//!
//! NOTE: Before opening a file in order to read information,
//! we need to give read access when opened a file. And then
//! we can use the read methods to read it entirely or partially
//!
//! Here, we can open the file using either of these methods with `read` permission:
//! - `File::open()`
//! - `OpenOptions::new()`. It has an edge over the former method as it gives multiple permissions
//!     at a time via chains like this...
//!     - 'r': `OpenOptions::new().read(true)`
//!     - 'rw': `OpenOptions::new().read(true).write(true)`
//!     - 'w': `OpenOptions::new().write(true)`
//!
//! We can also read the file:
//! - entirely via `read_to_end`
//! - partially via `read`

#[allow(unused_imports)]
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncReadExt;
use tokio::io::Result;

pub(crate) async fn main() -> Result<()> {
    // opening a file in read-only mode as permission
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
