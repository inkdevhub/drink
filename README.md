[![Rust checks](https://github.com/Cardinal-Cryptography/drink/actions/workflows/rust-checks.yml/badge.svg)](https://github.com/Cardinal-Cryptography/drink/actions/workflows/rust-checks.yml)
[![Built for ink!](https://raw.githubusercontent.com/paritytech/ink/master/.images/built-for-ink.svg)](https://github.com/paritytech/ink)

<h1 align="center"> DRink! </h1>
<p align="center"> Dechained Ready-to-play ink! playground </p>

https://github.com/Cardinal-Cryptography/drink/assets/27450471/4a45ef8a-a7ec-4a2f-84ab-0a2a36c1cb4e

# What is DRink?

DRink! aims providing support for ink! developers.
It comes in two parts:
1. `drink` library, which provides a minimal Substrate runtime allowing for ink! contracts development together with a facade interface for interacting with it
2. `drink-cli` command line tool, which puts `drink` behind friendly TUI

# DRink architecture

## Motivation

Actually, there are already some great tools for local contracts development like [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node) or [swanky](https://github.com/AstarNetwork/swanky-cli).
However, they all rely on a running node, which is not always convenient.
Especially in the early stage, when you want to quickly test your contract and don't want to bother with setting up a node, networking issues, block time, etc.

For testing purposes, ink! 4.x provides awesome framework, but in some complex cases it is still not enough.
For example, working with multiple contracts with different environments is not possible yet.
And still, unless you are working with off-chain execution, there's a node running behind the scenes.

## Solution

We work with literally minimal sufficient architecture that allows for a fully functional ink! contract development.
The only thing we need is a Substrate runtime with a contract pallet.
Having put it into `TextExternalities` we are ready to deploy and interact with contracts.
It's just like working within pallet's unit tests.

# How to use DRink?

You can use DRink in two ways.

### Directly as a library from your e2e tests

```rust
let mut sandbox = Sandbox::new();

let contract_bytes = fs::read("path/to/contract.wasm").unwrap();
let address = sandbox.deploy_contract(contract_bytes, compute_selector("new"), Default::default());
let result = sandbox.call_contract(address, compute_selector("foo"));
```

### Via `drink-cli` command line tool

When you run binary (`cargo run --release`) you'll see a DRink! TUI.

#### Managing mode

This is the default mode.
You can always enter it by pressing `Esc` key.
 - press `h` to see list of available commands with their brief descriptions
 - press `q` to quit TUI
 - press `i` to enter drinking mode

#### Drinking mode

This is the mode where you can interact with your environment.
 - you have `cd` and `clear` bashish commands available
 - `build` command will use your local `cargo-contract` tool to build a contract from the sources in the current directory
 - `deploy` command will deploy a contract from the current directory
 - by pressing `Tab` you can switch between all deployed contracts (with automatic directory change)
 - `call` command will call a contract with the given message
 - `next-block` command will advance the current block number
 - `add-tokens` command will add tokens to the given account
