//! Handle CRUD operations asynchronous in a file
//! - [x] Delete
//!
//! This example is on deleting file content
//! - entirely
//! - partially i.e. a particular line

use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::io::Result;

/// delete entire content
pub(crate) async fn delete_entirely() -> Result<()> {
    // if the file exists, then append
    if tokio::fs::try_exists("demo.txt").await? {
        // opening a file in append-only (append) mode as permission
        let mut file = OpenOptions::new().write(true).open("demo.txt").await?;

        // adding new content to existing content
        file.write_all(b"").await?;

        // throws error if not all the content written
        file.flush().await?;
    } else {
        panic!("Sorry! The file \"demo.txt\" doesn't exist.");
    }

    Ok(())
}

/// delete partial content
pub(crate) async fn delete_partially() -> Result<()> {
    todo!();
    Ok(())
}
