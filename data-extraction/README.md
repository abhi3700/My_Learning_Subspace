# Data Extraction

## Description

This is for extracting onchain data from the graphql.

## Queries

### Fetch 10 first/last blocks

```graphql
query {
  blocks(limit: 10, offset: 0, orderBy: height_DESC) {
      id
      hash
      height
      timestamp
      stateRoot
      blockchainSize
      spacePledged
      extrinsicsCount
      eventsCount
    }
}
```

> To fetch the 1st 10 blocks (from genesis), just change `orderBy` to `height_ASC`.

### Fetch the latest total space pledged

```graphql
query {
  blocks(limit: 10, offset: 0, orderBy: height_DESC) {
      spacePledged
    }
}
```

Just used the previous one with removing the unnecessary outputs.

### Get all the events with module names as case-insensitive

```graphql
query MyQuery {
  eventModuleNames(limit: 50, where: {name_containsInsensitive: "Reward"}) {
    name
  }
}
```

> There are multiple arguments like `name_eq`, `name_startsWith`, etc.

### Get all the blocks for a reward event

> ordered by descending timestamp to get the latest one first.

```graphql
query MyQuery {
  rewardEvents(limit: 50, where: {name_eq: "Rewards.BlockReward"}, orderBy: timestamp_DESC) {
    block {
      timestamp
    }
  }
}
```
