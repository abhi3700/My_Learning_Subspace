//! Get the latest total space pledged
//! & timestamp for voteReward at once.
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let graphql_query = GraphQLQuery {
        query: String::from(
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
        ),
        variables: json!({}),
    };

    let res = client
        .post("https://squid.gemini-3g.subspace.network/graphql")
        .json(&graphql_query)
        .send()
        .await?;

    let response_body: GraphQLResponse<BlocksResponse> = res.json().await?;
    // println!("{:?}", response_body);
    let bytes = response_body.data.blocks[0]
        .space_pledged
        .parse::<u128>()
        .expect("Not able to convert into u128");
    let timestamp: Option<i64> = if response_body.data.reward_events.is_empty() {
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
        "total space pledged: {} bytes, \ntimestamp: {}",
        bytes,
        timestamp.unwrap_or(0)
    );

    Ok(())
}
