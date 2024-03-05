# Multiple contracts

You can easily work with multiple contracts at the same time, even if they are not part of the same project.

Both `#[drink::contract_bundle_provider]` and `#[drink::test]` macros take care of building all the contract crates that you declare in `Cargo.toml`.
Therefore, even if you are testing a huge suite of dapps, the only thing you have to do is to run
```rust
cargo test --release
```

## Scenario

We will use PSP22 library as a dependency contract.
Simply declare it in `Cargo.toml`:
```toml
psp22 = { git = "https://github.com/Cardinal-Cryptography/PSP22.git", default-features = false, features = ["contract", "ink-as-dependency"] }
```

As usual, we have to include `ink-as-dependency` feature to use a contract as a dependency.
Moreover, we have to include `contract` feature to specify, that we are interested not only in the PSP22 standard trait, but actually in the default contract implementation.

Locally, we have a contract that keeps two addresses:
 - a deployed PSP22 contract's address
 - a user's address

The contract has a single message `check() -> u128`, which queries the PSP22 contract for the user's balance.
