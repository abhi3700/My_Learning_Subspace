//! Fetch 2 functions values by calling from runtime APIs exposed/defined in "sp_consensus_subspace" called "sp_api".
//! Likewise, there are several `decl_runtime_apis` in multiple modules. Just search for `decl_runtime_apis` in the `subspace` repository.
//! Source code: https://github.com/subspace/subspace/blob/5d8b65740ff054b01ebcbaf5a905e74274c1a5d0/crates/sp-consensus-subspace/src/lib.rs#L696-L753
//!
//! Pre-requisites:
//! 1. Run a subspace node.
//!     - Either use space-acres app.
//!     - Or use `subspace-node` CLI command.
//!
//! Then, you get to see this in terminal log:
//! ```sh
//! 2024-03-13T05:41:52.431057Z  INFO Node: sc_rpc_server: Running JSON-RPC server: addr=127.0.0.1:19944, allowed origins=["http://localhost:*", "http://127.0.0.1:*", "https://localhost:*", "https://127.0.0.1:*", "https://polkadot.js.org"]    
//! ```
//! Take the `addr`.
//!
//! 2. Generate subspace node metadata by:
//! ```sh
//! subxt metadata --url ws://127.0.0.1:19944 > metadata/subspace_metadata.scale
//! ```
//! before running this example.
//!
//! > By default, the url is `ws://127.0.0.1:9944`. But for custom, need to use `--url`.
//!
//!
//! NOTE: Better approach is now to use runtime_api wrapper which offloads the technical debt (using this approach). We should define via `decl_runtime_apis` (in pallet) & `impl_runtime_apis` (in runtime).
//! Example commit: Fetching solution ranges here: https://github.com/subspace/space-acres/blob/15532b86e5da32986a0e3ce447a57b6712b89d25/src/backend/node.rs#L300-L305

use subxt::config::PolkadotConfig;
use subxt::OnlineClient;

#[subxt::subxt(runtime_metadata_path = "metadata/subspace_metadata.scale")]
pub mod subspace {}

// Had to define separately as the inferred type differs from the original `sp_consensus_subspace::ChainConstants`.
type RuntimeChainConstants = subspace::runtime_types::sp_consensus_subspace::ChainConstants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The WebSocket URL of your synced node
    let url = "ws://127.0.0.1:19944";
    let api: OnlineClient<PolkadotConfig> = OnlineClient::<PolkadotConfig>::from_url(url).await?;

    let subspace_api: subspace::runtime_apis::subspace_api::SubspaceApi =
        subspace::apis().subspace_api();

    let solution_ranges_runtime_api_call = subspace_api.solution_ranges();
    // get the solution ranges from the latest block
    let current_solution_ranges = api
        .runtime_api()
        .at_latest()
        .await?
        .call(solution_ranges_runtime_api_call)
        .await?
        .current;
    println!("Current Solution Range: {}", current_solution_ranges);

    let chain_constants_runtime_api_call = subspace_api.chain_constants();
    // get the chain constants from the latest block
    let current_chain_constants = api
        .runtime_api()
        .at_latest()
        .await?
        .call(chain_constants_runtime_api_call)
        .await?;
    let RuntimeChainConstants::V0 {
        slot_probability, ..
    } = current_chain_constants;
    println!("Slot Probability: {:?}", slot_probability);

    Ok(())
}
