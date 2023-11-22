# Runtime interaction

This example shows how we can easily send transactions to a blockchain, just like with `subxt` or a similar client.

This way we can leverage `drink!` to a library for a full-scope runtime simulation engine.

## Running

```bash
cargo test --release
```

## `drink::Sandbox` vs `drink::Session`

While in most examples and showcases for `drink` you will see `drink::Session`, here we are using `drink::Sandbox`.
`Session` is very useful when you are working with contracts, but if you are focusing only on the runtime interaction, `Sandbox` is enough.
You can always switch from `Session` to `Sandbox` with:
```rust
    let session = Session::<Runtime>::new();
    ...
    let sandbox = session.sandbox(); // `sandbox` has type `&mut Sandbox<Runtime>`
```

A rule of thumb is: if you are working with contracts, use `Session`, otherwise use `Sandbox`.
