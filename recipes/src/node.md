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

### How to create a Subspace node?

```rust
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
