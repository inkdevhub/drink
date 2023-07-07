[![Rust checks](https://github.com/Cardinal-Cryptography/drink/actions/workflows/rust-checks.yml/badge.svg)](https://github.com/Cardinal-Cryptography/drink/actions/workflows/rust-checks.yml)
[![Built for ink!](https://raw.githubusercontent.com/paritytech/ink/master/.images/built-for-ink.svg)](https://github.com/paritytech/ink)

<h1 align="center"> DRink! </h1>
<p align="center"> Dechained Ready-to-play ink! playground </p>

https://github.com/Cardinal-Cryptography/drink/assets/27450471/4a45ef8a-a7ec-4a2f-84ab-0a2a36c1cb4e

# What is DRink?

DRink! aims to provide support for ink! developers.
It comes in two parts:
1. `drink` library, which provides a minimal Substrate runtime allowing for ink! contracts development together with a facade interface for interacting with it
2. `drink-cli` command line tool, which puts `drink` behind friendly TUI

# DRink architecture

## Motivation

Actually, there are already some great tools for local contracts development like [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node) or [swanky](https://github.com/AstarNetwork/swanky-cli).
However, they all rely on a running node, which is not always convenient.
Especially in the early stage, when you want to test your contract quickly and don't want to bother with setting up a node, networking issues, block time, etc.

For testing purposes, ink! 4.x provides an awesome framework, but in some complex cases, it is still not enough.
For example, working with multiple contracts with different environments is not possible yet.
And still, unless you are working with off-chain execution, there's a node running behind the scenes.

## Solution

We work with minimal sufficient architecture that allows for a fully functional ink! contract development.
The only thing we need is a Substrate runtime with a contract pallet.
Having put it into `TextExternalities` we are ready to deploy and interact with contracts.
It's just like working within pallet's unit tests.

# How to use DRink?

You can use DRink in two ways.

### Directly as a library from your e2e tests

```rust
let mut sandbox = Sandbox::new().unwrap();

let contract_bytes = fs::read("path/to/contract.wasm").unwrap();
let address = sandbox.deploy_contract(contract_bytes, compute_selector("new"), Default::default());
let result = sandbox.call_contract(address, compute_selector("foo"));
```

### Via the `drink-cli` command line tool

#### Dependencies

The only requirement for running DRInk! is having Rust installed. The code was tested with version `1.69.0-nightly`. All other dependencies are managed by Cargo and will be installed upon running `cargo build` or `cargo run`.

#### Running DRInk! CLI

When you run the binary (`cargo run --release`) you'll see a DRink! TUI. You can also choose to start from a specific path by supplying the `--path` argument like:
```bash
cargo run --release --path example/flipper
```

### CLI modes

In a somewhat Vim-inspired way, the `drink-cli` allows you to work in two modes: the Managing mode and the Drinking mode. 

#### Managing mode

This is the default mode, facilitating high-level interactions with the TUI itself.
At any point, you can enter it by pressing the `Esc` key. Once in the Managing mode:
 - Press `h` to see a list of available commands with their brief descriptions;
 - Press `q` to quit the TUI;
 - Press `i` to enter the Drinking mode.

#### Drinking mode

This is the mode where you can interact with your environment and type your commands inside the `User input` field. When in Managing mode, you can enter the Drinking mode by pressing 'i' on your keyboard.
You have several commands at your disposal (you can list them in the Managing mode by pressing the 'h' key):
 - `cd` and `clear` will, just like their Bash counterparts, change the directory and clear the output, respectively. You will see the current working directory as the first entry in the `Current environment` pane;
 - `build` command will build a contract from the sources in the current directory;
 - `deploy` command will deploy a contract from the current directory. Note that if your constructor takes arguments, you will need to supply them to this command, like: `deploy true` in the case of the Flipper example;
 - by pressing `Tab` you can switch between all deployed contracts (with automatic directory change);
 - `call` command will call a contract with the given message. Again, if the message takes arguments, they need to be supplied here;
 - `next-block` command will advance the current block number;
 - `add-tokens` command will add tokens to the given account.
