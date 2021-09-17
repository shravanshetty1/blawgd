# [blawgd.com](https://blawgd.com)

Blawgd is a censorship resistant micro-blogging platform.

### Overview
It achieves censorship resistance and decentralization from blockchain technology. A new post, like, follow .etc. are all 
stored on the blockchain.

The light clients (website and apps) verify all data received from nodes of the blockchain network using [merkle proofs](https://en.wikipedia.org/wiki/Merkle_tree)
, making it impossible for any man in the middle to censor information without being detected.

(Currently unimplemented) - All nodes on the network communicate over [TOR](https://en.wikipedia.org/wiki/Tor_(network))
 making the network highly resistant to takedowns from external entities.

You can find out more details by going through the architecture [documentation](./docs/ARCHITECTURE.md).

### Notable features
1. Censorship proof
2. Heavily decentralized
3. Resitant to takedowns (unimplemented)
4. Incentivizes popular posts.

### Status
The network has been released where as the clients and providers are under active development.

### Platforms
Currently, a rudimentary website client has been released and is available [here](https://blawgd.com). Apps should be released soon which will be a lot
more decentralized than the website version.

### Support
Feel free to create issues for any bugs you encounter and document reliable ways to reproduce them.