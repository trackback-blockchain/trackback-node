# TrackBack Node Limitations (Minimum Viable Product).


## Disclaimer
* This node created as minimum viable product
* Do not use ina  production environment
* Released for test purposes only

## Limitations
* No [token](https://docs.substrate.io/how-to-guides/v3/basics/mint-token/) economic models around managing Decentralised Identifiers
* No staking rewards
* Default configuration has 2 validators and a not well known node ( 3 node network )
* OnChain data will recycle after 6 weeks ( subject to change )
* Does not have  the complete functionality for [DIDComms](https://identity.foundation/didcomm-messaging/spec/)
* Covers the functionality for creation, revocation, update and retrieve a DID only
* Provides support for Ledger based DIDs only. 
* Does not cover Ledger Middleware DIDs, Peer DIDs, Static DIDs and Alternative DIDs
* Tight bindings between a Controller and the Chain  limited to default accounts
