# Cross Contract Calls
In this section, we will learn cross-contract calls by deploying vending-machine contract which interacts with pre-deployed FT contract.
* Docs Reference: [Cross-Contract Calls](https://docs.near.org/develop/contracts/crosscontract)
* Github Reference: [`near-examples/docs-examples`](https://github.com/near-examples/docs-examples/tree/main/cross-contract-hello-rs)

![image](https://user-images.githubusercontent.com/96561121/187244014-19e7a22c-0315-4454-8601-af2041333b4f.png)
This is a diagram explains what we build.
1) User calls `get_token` method of `vendor.sender.testnet` contract.
2) `vendor.sender.testnet` contract calls `ft_mint` method of `token.sender.testnet` with `sender.testnet` & `amount` as arguments.
3) `token.sender.testnet` mints token for `sender.testnet`. as result of `ft_mint` method, FT contract returns `promise` containing `"{ amount } tokens for { sender.testnet } are minted”` message.
4) `vendor.sender.testnet` contract executes `callback` method to process received `promise`.
5) User gets error message or `"{ amount } tokens for { sender.testnet } are minted”` message.

## Let's move to code!
```bash
git checkout 5.contract/cross-contract-call
```
Checkout to 5th branch.   

```
near-meetup
├─ token
│  ├─ src
│  │   └─ lib.rs
│  └─ target
│
├─ vending-machine
│  ├─ src
│  │  ├─ external_traits.rs
│  │  └─ lib.rs
│  └─ target
│
└─ export
   ├─ token.wasm
   └─ vending-machine.wasm
```
This is hierarchy of file system. Code of FT contract is stored in `near-meetup/token/src/`, and code of vending-machine is stored in `near-meetup/vending-machine/src/`. And compiled wasm files are stored in `near-meetup/export/`.
## NEXT STEP: [Cross Contract Call](https://github.com/boomlabs-web3/near-meetup/tree/5.contract/cross-contract-call)
