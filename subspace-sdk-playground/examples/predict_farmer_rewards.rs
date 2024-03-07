//! Add sample code for predicting farmer rewards.
//!
//! Notion: https://www.notion.so/subspacelabs/Farmer-s-Reward-Prediction-Space-Acres-f420a82c67324028a2277affafd88e6e?pvs=4
//!
//! ## A-1: **Proportional Contribution Approach (Simple)**
//!
//! This approach is mainly based on the farming contribution ratio (**CR**), which can be calculated as:
//!
//! $$
//! contribution\_ratio = \frac{\text{space\_pledged}}{\text{total\_space\_pledged}}
//! $$
//!
//! where,
//!  `total_space_pledged`: Total Space Pledged in the network
//!  `space_pledged`: Farmer's Space Pledged
//!
//! Now, we can get the timestamp of the last farming reward payment from on-chain data.
//! This is required to get the expected timestamp of the next farming reward payment using farmer’s `contribution_ratio` ****value**.** So, the formula is:
//!
//! $$
//! expected\_time\_next = \frac{\text{total\_space\_pledged}}{\text{space\_pledged}} * time\_previous
//! $$
//!
//! where,
//!  `expected_time_next`: Expected time duration for next reward payment since the last reward payment timestamp.
//!  `time_previous`: Time elapsed since the last reward payment timestamp.
//!
//! Therefore, the farmer can expect to get the next reward payment in (`expected_time_next` **-** `time_previous`) time units (sec/min/hr).

use chrono::{DateTime, Utc};
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
    #[serde(rename = "rewardEvents")]
    reward_events: Vec<RewardEvent>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    #[serde(rename = "spacePledged")]
    space_pledged: String,
}

#[derive(Deserialize, Debug)]
struct RewardEvent {
    block: BlockTimestamp,
}

#[derive(Deserialize, Debug)]
struct BlockTimestamp {
    timestamp: String,
}

/// Get response body from a graphql query
async fn get_response_body(query: String) -> eyre::Result<GraphQLResponse<BlocksResponse>> {
    let client = Client::new();
    let graphql_query = GraphQLQuery {
        query,
        variables: json!({}),
    };

    let res = client
        .post("https://squid.gemini-3g.subspace.network/graphql")
        .json(&graphql_query)
        .send()
        .await?;

    Ok(res.json().await?)
}

/// the farmer can expect to get the next reward payment in (expected_time_next - time_previous) time units (sec/min/hr).
fn calculate_expected_reward_duration_from_now(
    total_space_pledged: u128,
    space_pledged: u128,
    last_reward_timestamp: Option<i64>,
) -> i64 {
    // Time elapsed since the last reward payment timestamp.
    let time_previous = Utc::now().timestamp() - last_reward_timestamp.unwrap_or(0);

    // Expected time duration for next reward payment since the last reward payment timestamp.
    let expected_time_next = (total_space_pledged as i64 / space_pledged as i64) * time_previous;

    expected_time_next - time_previous
}

/// TODO: Create as high as minutes, hours, days, etc.

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let query = String::from(
        r#"query {
            blocks(limit: 1, offset: 0, orderBy: height_DESC) {
                spacePledged
            }
            rewardEvents(limit: 1, where: {name_eq: "Rewards.VoteReward"}, orderBy: timestamp_DESC) {
                block {
                    timestamp
                }
            }            
        }"#,
    );

    let response_body: GraphQLResponse<BlocksResponse> = get_response_body(query).await?;
    let total_space_pledged = response_body.data.blocks[0]
        .space_pledged
        .parse::<u128>()
        .expect("Not able to convert into u128");
    let last_reward_timestamp: Option<i64> = if response_body.data.reward_events.is_empty() {
        None
    } else {
        Some(
            response_body
                .data
                .reward_events
                .get(0)
                .expect("Unable to get reward event")
                .block
                .timestamp
                .parse::<DateTime<Utc>>()
                .expect("Not able to convert into timestamp")
                .timestamp(),
        )
    };

    println!(
        "total space pledged: {} bytes, \nLast reward timestamp: {}",
        total_space_pledged,
        last_reward_timestamp.unwrap_or(0)
    );

    // let space_pledged = 2_147_483_648; // 2 GiB in bytes
    // let space_pledged = 100_000_000_000; // 100 GB in bytes
    let space_pledged = 2_199_023_255_552; // 2 TiB in bytes

    println!(
        "The farmer can expect to get the next reward payment in another {} seconds.",
        calculate_expected_reward_duration_from_now(
            total_space_pledged,
            space_pledged,
            last_reward_timestamp
        )
    );

    Ok(())
}
