# Keypair

## Overview

This chapter is all about keypair. It contains code snippets and functions that can be used to create a keypair.

## Recipes

### How to create a public key?

```rust
use subspace_sdk::PublicKey;

// create a 32 bytes public key
let x = PublicKey::from([1; 32]);
println!("{:?}", x.0);
```

### How to create a secret key?

<!-- TODO: -->
