# Runtime interaction

This example shows how we can easily send transactions to a blockchain, just like with `subxt` or a similar client.

This way we can leverage `drink!` to a full-scope runtime simulation engine.

## Running

```bash
cargo test --release
```

## `drink::Sandbox` vs `drink::Session`

While in most examples and showcases for `drink` you will see `drink::Session`, here we are using the associated `Sandbox` implementation directly.
`Session` is very useful when you are working with contracts, but if you are focusing only on the runtime interaction, you can simply use the underlying `Sandbox`.
You can get a reference to the `Sandbox` implementation from the `Session` object using the `sandbox` method:

```rust
    let session = Session::<MinimalSandbox>::default();
    ...
    let sandbox = session.sandbox(); // `sandbox` has type `&mut MinimalSandbox`
```

`Sandbox` is just a runtime wrapper, which enables you to interact directly with the runtime.
On the other hand, `Session` is a wrapper around `Sandbox` which also provides a useful context for working with contracts.

A rule of thumb is: if you are working with contracts, use `Session`, otherwise use `Sandbox`.
