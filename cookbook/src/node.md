# Node

## Overview

This chapter contains code snippets and functions that can be used to build a Subspace node.

## Recipes

### What are the local dependencies for creating a subspace node?

Here are the crates that are used to create a subspace node:

```toml
sdk-substrate
sdk-node
```

### How a subspace node type is defined?

```rust
use subspace_proof_of_space::chia::ChiaTable;

/// Subspace farmer type
pub type Farmer = sdk_farmer::Farmer<ChiaTable>;

/// Subspace primary node
pub type Node = sdk_node::Node<Farmer>;
```

### How to create a simple Subspace node?

```rust
//! Source code at file: https://github.com/subspace/subspace-sdk/blob/main/examples/simple.rs

// import the required crates
use subspace_sdk::Node;
use sdk_node::PotConfiguration;

let node = subspace_sdk::Node::builder()
    .force_authoring(true)
    .role(subspace_sdk::node::Role::Authority)
    // Starting a new chain
    .build(
        "node",
        subspace_sdk::chain_spec::dev_config(),
        PotConfiguration { is_pot_enabled: false, is_node_time_keeper: true },
    )
    .await
    .unwrap();
```

### How to create an advanced Subspace node with some network option to listen for peers' addresses?

```rust
//! Source code at file: https://github.com/subspace/subspace-sdk/blob/main/examples/sync.rs

// import the required crates
use subspace_sdk::Node;
use subspace_sdk::node::NetworkBuilder;
use sdk_node::PotConfiguration;

// create a node
let node = Node::builder()
    .network(
        // create a network builder which is defined as struct - `Network` to be called as `NetworkBuilder`
        NetworkBuilder::new()
            .listen_addresses(vec!["/ip4/127.0.0.1/tcp/0".parse().unwrap()])
            .force_synced(true),
    )
    .force_authoring(true)
    .role(subspace_sdk::node::Role::Authority)
    .build(
        // this `node` is of PathBuf type
        node,
        // `chain_spec` is defined in the `chain_spec.rs` file which needs to be read.
        chain_spec,
        PotConfiguration { is_pot_enabled: false, is_node_time_keeper: true },
    )
    .await?;
```

### How to stop a node to author blocks after running few blocks?

```rust
// define a node
// === code from `examples/simple.rs` ===

// stop authoring blocks after running 10 blocks
node.subscribe_new_heads()
    .await
    .unwrap()
    // Wait 10 blocks and exit
    .take(10)
    .for_each(|header| async move { tracing::info!(?header, "New block!") })
    .await;
```

Here, `take(10)` method will stop authoring blocks after running 10 blocks.

In order to view fresh block production, you have to remove the `parity_db` folder, else it will stop producing blocks after 10 blocks like `0` -> `9` and then `9` -> `18` and `18` -> `27` and so on.

> Please note here that the block which the previous iteration ended at, the same block number the next iteration will start from. E.g. Like in this case, run-1 ended at `9` block, run-2 will start from `9` block and end at `18` block and so on. So, the total blocks produced is 10 in each iteration.

```sh
$ rm -rf /path/to/parity_db
```

### How to set a chain spec for a node?
