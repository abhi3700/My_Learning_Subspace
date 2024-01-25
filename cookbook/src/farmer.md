# Farmer

## Overview

## Recipes

### How a farmer type is defined?

```rust
use subspace_proof_of_space::chia::ChiaTable;

/// Subspace farmer type
pub type Farmer = sdk_farmer::Farmer<ChiaTable>;
```

### How to define a farm/plot?

A farm is defined via a `FarmDescription` struct. It contains the following fields:

- `directory`: Path of the farm
- `space_pledged`: Space which you want to pledge for the farm

```rust
//! Source code at file: https://github.com/subspace/subspace-sdk/blob/main/farmer/src/lib.rs

/// Description of the farm
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct FarmDescription {
    /// Path of the farm
    pub directory: PathBuf,
    /// Space which you want to pledge
    pub space_pledged: ByteSize,
}
```

- Here, the struct can be printed/logged, cloned, compared for equality, deserialized and serialized.
- The struct is non-exhaustive, which means that the struct may have additional fields added to it in the future without breaking backwards compatibility. It is a way to signal to users of the struct that they should not rely on the struct having only the fields defined in its current version.

### What are the associated methods of a farm/plot?

A farm has the following associated methods:

- `new`: Construct Farm description
- `wipe`: Wipe all the data from the farm

The functions are defined like this:

```rust
//! Source code at file: https://github.com/subspace/subspace-sdk/blob/main/farmer/src/lib.rs

impl FarmDescription {
    /// Construct Farm description
    pub fn new(directory: impl Into<PathBuf>, space_pledged: ByteSize) -> Self {
        Self { directory: directory.into(), space_pledged }
    }

    /// Wipe all the data from the farm
    pub async fn wipe(self) -> io::Result<()> {
        tokio::fs::remove_dir_all(self.directory).await
    }
}
```

### How to farm?

This has been abstracted in the `pulsar` tool. The `farm` command is responsible for farming. [Chapter](./pulsar/farm.md).
