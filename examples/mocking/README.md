# Mocking contracts

This example shows how we can easily mock contracts with the `drink!` library.

## Scenario

Say we want to test a contract that simply forwards call to another contract (i.e. a _proxy_ pattern).
Our contract has a single message `forward_call(AccountId) -> (u8, u8)`.
We want to test that this proxy correctly calls the callee (with some fixed selector) and returns the unchanged result (a pair of two `u8`).

Normally, we would have to implement and build a mock contract that would be deployed alongside the tested contract.
With drink, we can simply mock the logic with some closures and test our contract in isolation.

## Running

```bash
cargo contract build --release
cargo test --release
```
