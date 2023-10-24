//! Handle CRUD operations asynchronous in a file
//! - [x] Update
//! Update means it could be:
//! - adding new content to existing content
//! - replacing existing content with a new one

use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::io::Result;

/// adding new content to existing content
pub(crate) async fn update_add_new_content() -> Result<()> {
    // if the file exists, then append
    if tokio::fs::try_exists("demo.txt").await? {
        // opening a file in append-only (append) mode as permission
        let mut file = OpenOptions::new().append(true).open("demo.txt").await?;

        // adding new content to existing content
        file.write_all(b"\nThe project is very exciting! It has potentially the best consensus algorithm \nobserved so far ensuring the Blockchain trilemma.").await?;

        // throws error if not all the content written
        file.flush().await?;
    } else {
        panic!("Sorry! The file \"demo.txt\" doesn't exist.");
    }

    Ok(())
}

/// replacing existing content with a new one
pub(crate) async fn update_replace_ewn_content() -> Result<()> {
    // if the file exists, then append
    if tokio::fs::try_exists("demo.txt").await? {
        // opening a file in write-only (append) mode as permission
        let mut file = OpenOptions::new().write(true).open("demo.txt").await?;

        // adding new content to existing content
        file.write_all(b"The project is very exciting! It has potentially the best consensus algorithm \nobserved so far ensuring the Blockchain trilemma.").await?;

        // throws error if not all the content written
        file.flush().await?;
    } else {
        panic!("Sorry! The file \"demo.txt\" doesn't exist.");
    }

    Ok(())
}
