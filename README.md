# near_meetup
github repository for near meetup in boomlabs
## Additional Material
* [Cheat Sheet](https://bit.ly/near_meetup) for Near Protocol Hands-on workshop.
* [[NEAR 102] Part 1: Understanding Near Protocols — Mainnet Architecture and RPC Calls](https://medium.com/dsrv/near-102-understanding-near-protocol-mainnet-architecture-and-rpc-call-75351d28bdb4), DSRV medium post written in Korean corresponding to Day 1.

## Overview
| Branch                                                                                                               | Docs Tutorial                                                                                                  | Description                                                                                                                                            | Reference                                                                                                         |
| ---------------------------------------------------------------------------------------------------------------------| ---------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------|
| **Day 1**                                                                                                            |                                                                                                                |                                                                                                                                                        |                                                                                                                   |
| [`1.rpc/near-api-js`](https://github.com/boomlabs-web3/near-meetup/tree/1.rpc/near-api-js)                           | [Create a Transaction](https://docs.near.org/integrator/create-transactions#low-level----create-a-transaction) | Interact with NEAR blockchain by sending 1 NEAR in 3 ways (by using near-cli, near-api-js, near-api-js & postman)                                      | [`near-examples/transaction-examples`](https://github.com/near-examples/transaction-examples)                     |
| [`2.contract/template`](https://github.com/boomlabs-web3/near-meetup/tree/2.contract/template)                       | [Structure of a Contract](https://www.near-sdk.io/contract-structure/near-bindgen)                             | Learn about basic NEAR contract structure written in rust.                                                                                             | [`near-examples/rust-counter`](https://github.com/near-examples/rust-counter)                                     |
| [`3.contract/simple-ft`](https://github.com/boomlabs-web3/near-meetup/tree/3.contract/simple-ft)                     | [Fungible Tokens](https://docs.near.org/develop/relevant-contracts/ft)                                         | Deploy & Initialize simple fungible token contract in NEAR testnet.                                                                                    | [`near-examples/FT`](https://github.com/near-examples/FT)                                                         |
| **Day 2**                                                                                                            |                                                                                                                |                                                                                                                                                        |                                                                                                                   |
| [`4.contract/upgraded-ft`](https://github.com/boomlabs-web3/near-meetup/tree/4.contract/upgraded-ft)                 | [Schema Migration](https://welcome.near.university/developers/contract-patterns/schema-migration)                                     | Upgrade previously deployed FT contract in 3rd branch, and learn about schema migration.                                                               |                                                                                                                   |
| [`5.contract/cross-contract-call`](https://github.com/boomlabs-web3/near-meetup/tree/5.contract/cross-contract-call) | [Cross-Contract Calls](https://docs.near.org/develop/contracts/crosscontract)                                  | Learn cross-contract calls by deploying vending-machine contract which interacts with FT contract above.                                               | [`near-examples/docs-examples`](https://github.com/near-examples/docs-examples/tree/main/cross-contract-hello-rs) |
| [`6.contract/test`](https://github.com/boomlabs-web3/near-meetup/tree/6.contract/test)                               | [Test](https://docs.near.org/develop/testing/introduction)                                                     | Learn unit test & integration test in NEAR. Learn how to mock blockchain environment in local environment, spooning contracts from testnet or mainnet. | [`near/workspaces-rs`](https://github.com/near/workspaces-rs)                                                     |

## Prerequisites

### Install Node.js
```=bash
brew install node
npm install --global yarn
```
### Install Rust & Wasm
```=bash
# Install Rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

# Add Wasm toolchain
rustup target add wasm32-unknown-unknown
```
### Install near-cli
```=bash
# node version should be above 12
npm install -g near-cli
```

### Install near-api-js
```=bash
npm i --save near-api-js
```

### Install ts-node
```=bash
npm install -g typescript
npm install -g ts-node
```
### [Create Testnet Wallet](https://wiki.near.org/getting-started/creating-a-near-wallet)
### [Postman Setup](https://docs.near.org/api/rpc/setup#postman-setup)
