# [Blawgd](https://blawgd.com)

Blawgd is a censorship resistant micro-blogging platform.

### Platforms
Currently, the proof of concept website has been released and is available [here](https://blawgd.com). Apps should be released soon which will be a lot
more decentralized than the website version.

The application is not ready for casual use yet and data maybe wiped at any time without notice until official release.

### Overview
It achieves censorship resistance and decentralization from blockchain technology. A new post, like, follow .etc. are all added
to the blockchain. 

The light clients (website and apps) verify all data received from nodes using [merkle proofs](https://en.wikipedia.org/wiki/Merkle_tree)
, making it impossible for any man in the middle to censor information without being detected.

(Currently unsupported) - All nodes on the network communicate over [TOR](https://en.wikipedia.org/wiki/Tor_(network)) making the network highly resistant
 to takedowns from entities such as corrupt governments and agencies.

You can find out more details by going through the architecture [documentation](./docs/ARCHITECTURE.md).