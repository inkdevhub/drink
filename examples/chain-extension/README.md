# Testing a chain extension

This example shows how you can use `drink` to test a chain extension.

From the perspective of a contract implementation or writing tests there is nothing special (i.e., more than usual) that you have to do to interact with a chain extension.
The thing that `drink` makes easier for you is combining an arbitrary chain extension with `drink`'s `MinimalRuntime`.
By simply calling:
```rust
create_minimal_runtime!(
    RuntimeWithCustomChainExtension,
    path::to::MyCustomChainExtension
);
```

you are provided with a runtime that contains your custom chain extension and can be used to test your contract like:
```rust
Session::<RuntimeWithCustomChainExtension>::new()?
    .deploy_bundle_and(...)?
    .call(...)?
```
