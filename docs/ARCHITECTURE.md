# Blawgd architecture

### Overview

This document aims to explain the inner workings of the blawgd network allowing the reader to understand why the platform is
censorship resistant. 
Basic understanding of [blockchain](https://en.wikipedia.org/wiki/Blockchain) will help greatly in readers ability to comprehend this document.
This document will not explain blockchain concepts, however will provide links to other documents that explain these concepts.

Blawgd is similar to other microblogging platforms such as [twitter](https://twitter.com) and [koo](https://kooapp.com) in 
its usage however it greatly differs from them in its inner workings.
The other popular microblogging platform store data on their servers which are controlled by them
giving them complete control of what goes on in their platform. This makes it very easy for owners
of the platform to control what the user sees and is allowed to say on their platform. They are
able to ban anyone they want and artificially improve visibility of opinions they favor.

The blawgd platform on the other hand
 is controlled by various entities such as stakeholders of the blockchain network and client authors
making it very hard for any one party to influence the behaviour of the platform.
The blawgd platform is divided into the network and the clients. The network stores all the data
and the client verifies the data queried from the network.

The blawgd network is controlled by stake holders of the blockchain network using [proof of stake](https://en.wikipedia.org/wiki/Proof_of_stake).
Any malicious change to the network has to be voted on by a majority of stakeholders democratically
which makes it hard for any one party to censor information.

The blawgd clients are created by various client authors and will be [open source](https://en.wikipedia.org/wiki/Open_source).
Any malicious change to the client has to be made by the author of the client
 and will be immediately detected by the community since its [open source](https://en.wikipedia.org/wiki/Open_source).
Since each client will be used by fraction of the user base any potential censorship for a period of time
 will have insignificant influence over the platform.
 
### Notable differences
 To create blawgd platform certain tradeoffs had to be made making the user experience of using blawgd different from
 using traditional microblogging platforms. Here are some of the notable differences.
 1. Old posts are not stored on the network (they will be stored off chain by "[providers](#providers)").
 2. Likes work differently, a like sends a coin from the user to the post author. A user can like
 a post multiple times sending multiple coins.
 3. Users can repost multiple times. Old posts can be kept alive after deletion by reposts.
 4. Search is not provided by the network (will be provided by "providers")
 5. Notifications are not provided by the network (will be provided by "providers")
 6. Recommendations are not provided by the network (will be provided by "providers")
 7. Login mechanism may be drastically different than traditional login mechanisms.
 8. Reads are free but any action that requires writes such as creating a new post or liking an existing posts
 require some coins to be paid as transaction fees. The transaction fee will vary based on the traffic of the network.

### The network

The blawgd network is built using the [cosmos-sdk](https://github.com/cosmos/cosmos-sdk) framework. This section will
go through blawgd specific features built on top of the features provided out of the box by cosmos-sdk. This section
will not cover cosmos-sdk specific features. If your interested in more details about the network you can read 
the [cosmos docs](https://docs.cosmos.network/) and the [tendermint docs](https://docs.tendermint.com/#).

A computer on the network will be referred to as a node.

The blawgd network grpc api is present [here](../backends/cosmos/api/grpc/blawgd.proto). Each node on the network stores
all the data related to blawgd platform. Transactions are used to add/modify data present in these nodes. A user can
create transactions using clients and can send the transaction to the network which will modify the data present in the nodes.
This is not blawgd specific and is common in all blockchains so we wont cover more details here. Please refer to cosmos docs for more details.

The transactions supported by blawgd network are -
```
// Transactions

message MsgRepost {
  string creator = 1;
  string post_id = 2;
}

message MsgCreatePost {
  string creator = 1;
  string content = 2;
  string parent_post = 3;
}

message MsgUpdateAccountInfo{
  string creator = 1;
  string name = 2;
  string photo = 3;
}

message MsgFollow {
  string creator = 1;
  string address = 2;
}

message MsgStopFollow {
  string creator = 1;
  string address = 2;
}

message MsgLikePost {
  string creator = 1;
  string post_id = 2;
  uint64 amount = 3;
}
```

Each of these transactions modifies data on the network in a specific way which is self explanatory(MsgCreatePost - creates a new
post .etc.).

Each node on the network exposes the "Get" grpc endpoint which can be queried to get various data stored on the network
along with [merkle proofs](https://en.wikipedia.org/wiki/Merkle_tree) mathematically proving existance or non existance of data.

Each node on the networks stores in an [IAVL+ tree](https://github.com/cosmos/iavl) allowing it to provide merkle proofs.

```
service Query {
  rpc Get(GetRequest) returns (GetResponse);
}

message GetRequest {
  uint64 height = 1;
  repeated string keys = 2;
}

message GetResponse {
  map<string,bytes> data = 1;
  map<string,bytes> proofs = 2;
}
```

Each node on the network only stores the data related to the last n posts. This n value is set during the genesis of the network in the genesis file.
```
message GenesisState {
  uint64 max_post_count = 1;
}
```
The `max_post_count` will be increased as network experiences more traffic such 
that the network is always storing the last 30-15 days worth of posts.

All nodes on the network should communicate over [TOR](https://www.torproject.org/), however currently they dont. This is planned in future development.

The current network is built using [cosmos-sdk](https://github.com/cosmos/cosmos-sdk) however we will continue to 
build other network implementations with technologies such as [substrate](https://github.com/paritytech/substrate)
and [solana smart contracts](https://github.com/solana-labs/solana) to keep up with advancements in blockchain technology.


### The clients

Clients are software that can retrieve data from the blawgd network and verify the authenticity of the data using [merkle proofs](https://en.wikipedia.org/wiki/Merkle_tree).

Currently we have built a browser based client written in rust that uses the cosmos rust light client
implementation which can be found [here](https://github.com/informalsystems/tendermint-rs). More details
on how the light client works can be found [here](https://docs.tendermint.com/master/spec/light-client/).

To summarize the current client gets block headers from nodes using the [light client protocol](https://docs.tendermint.com/master/spec/light-client/).
It then retrieves data from nodes in the blawgd network along with proofs. It verifies the proofs against the block headers to
verify the authenticity of the data provided by the node. Once the data is proved to be authentic the data is parsed
and displayed to the user as posts, account info, likes .etc.

This ensures that even though the nodes on the network can be controlled by malicious parties 
they cannot censor information since they have to provide proof of existance or non existance of data. Any tampering by the node
will be detected by the light client.

This document also aims to encourage users to become potential client authors, this would make the platform more decentralized.

You can find the current POC client [here](https://blawgd.com) with apps coming soon.


### Providers

Due to blockchain technologies certain features cannot be supported on the blockchain in a censorship free and decentralized 
way but may still be demanded by users. To support these features we plan on implementing "providers" 
which will provide these features to the client without any promises of censorship resistance.

Some of these features include -
1. Search
2. Notifications
3. Recommendations
4. Unique likes
5. Old posts

These features should not be trusted by the user as they cannot be proved to be authentic using merkle proofs.
The client authors should aim to implement their clients in such a way that if a "provider" has been taken down
it shouldnt affect the core features of the application.

This is yet to be implemented.