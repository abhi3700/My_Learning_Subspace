//! Get realtime value of "Total Space Pledged (TSP)"
//! GraphQL url for this: https://squid.gemini-3g.subspace.network/graphql
//!
//! This is for a block:
//! ```
//! query {
//!   blocks(limit: 1, offset: 0, orderBy: height_DESC) {
//!       id
//!       hash
//!       height
//!       timestamp
//!       stateRoot
//!       blockchainSize
//!       spacePledged
//!       extrinsicsCount
//!       eventsCount
//!     }
//! }
//! But, it can be reduced to
//! query {
//!   blocks(limit: 1, offset: 0, orderBy: height_DESC) {
//!       spacePledged
//!     }
//! }
//! as we only need `spacePledged`.
//! ```

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct GraphQLQuery {
    query: String,
    variables: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct GraphQLResponse<T> {
    data: T,
}

#[derive(Deserialize, Debug)]
struct BlocksResponse {
    blocks: Vec<Block>,
}

#[derive(Deserialize, Debug)]
struct Block {
    #[serde(rename = "spacePledged")]
    space_pledged: String,
}

fn convert_to_highest_unit(bytes: u128) -> String {
    let unit = 1024u128.pow((bytes.next_power_of_two().trailing_zeros() / 10) as u32);
    let converted = bytes as f64 / unit as f64;
    let unit_str = match unit {
        1_024 => "KiB",
        1_048_576 => "MiB",
        1_073_741_824 => "GiB",
        1_099_511_627_776 => "TiB",
        1_125_899_906_842_624 => "PiB",
        _ => unreachable!(),
    };
    format!("{:.2} {}", converted, unit_str)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let graphql_query = GraphQLQuery {
        query: String::from(
            r#"query {
                blocks(limit: 1, offset: 0, orderBy: height_DESC) {
                    spacePledged
                }
            }"#,
        ),
        variables: json!({}),
    };

    let res = client
        .post("https://squid.gemini-3g.subspace.network/graphql")
        .json(&graphql_query)
        .send()
        .await?;

    let response_body: GraphQLResponse<BlocksResponse> = res.json().await?;
    println!("{:?}", response_body);
    let bytes = response_body.data.blocks[0]
        .space_pledged
        .parse::<u128>()
        .expect("Not able to convert into u128");
    convert_to_highest_unit(bytes);
    println!("total space pledged: {}", convert_to_highest_unit(bytes));

    Ok(())
}
