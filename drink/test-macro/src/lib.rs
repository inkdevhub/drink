//! Procedural macro providing a `#[drink::test]` attribute for `drink`-based contract testing.

#![warn(missing_docs)]

mod bundle_provision;
mod contract_building;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ItemEnum, ItemFn};

use crate::contract_building::build_contracts;

type SynResult<T> = Result<T, syn::Error>;

/// Defines a drink!-based test.
///
/// # Requirements
///
/// - You mustn't import `drink::test` in the scope, where the macro is used. In other words, you
/// should always use the macro only with a qualified path `#[drink::test]`.
/// - Your crate cannot be part of a cargo workspace.
///
/// # Impact
///
/// This macro will take care of building all needed contracts for the test. The building process
/// will be executed during compile time.
///
/// Contracts to be built:
///  - current cargo package if contains a `ink-as-dependency` feature
///  - all dependencies declared in the `Cargo.toml` file with the `ink-as-dependency` feature
/// enabled
///
/// Note: Depending on a non-local contract is not tested yet.
///
/// # Example
///
/// ```rust, ignore
/// #[drink::test]
/// fn testcase() {
///     Session::<MinimalRuntime>::new()
///         .unwrap()
///         .deploy(bytes(), "new", NO_ARGS, vec![], None, &transcoder())
///         .unwrap();
/// }
/// ```
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    match test_internal(attr.into(), item.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Auxiliary function to enter ?-based error propagation.
fn test_internal(_attr: TokenStream2, item: TokenStream2) -> SynResult<TokenStream2> {
    let item_fn = syn::parse2::<ItemFn>(item)?;
    build_contracts();

    Ok(quote! {
        #[test]
        #item_fn
    })
}

/// Defines a contract bundle provider.
///
/// # Requirements
///
/// - Your crate cannot be part of a cargo workspace.
/// - Your crate must have `drink` in its dependencies (and it shouldn't be renamed).
/// - The attributed enum must not:
///     - be generic
///     - have variants
///     - have any attributes conflicting with `#[derive(Copy, Clone, PartialEq, Eq, Debug)]`
///
/// # Impact
///
/// This macro is intended to be used as an attribute of some empty enum. It will build all
/// contracts crates (with rules identical to those of `#[drink::test]`), and populate the decorated
/// enum with variants, one per built contract.
///
/// If the current crate is a contract crate, the enum will receive a method `local()` that returns
/// the contract bundle for the current crate.
///
/// Besides that, the enum will receive a method `bundle(self)` that returns the contract bundle
/// for corresponding contract variant.
///
/// Both methods return `DrinkResult<ContractBundle>`.
///
/// # Example
///
/// ```rust, ignore
/// #[drink::contract_bundle_provider]
/// enum BundleProvider {}
///
/// fn testcase() {
///     Session::<MinimalRuntime>::new()?
///         .deploy_bundle_and(BundleProvider::local()?, "new", NO_ARGS, vec![], None)
///         .deploy_bundle_and(BundleProvider::AnotherContract.bundle()?, "new", NO_ARGS, vec![], None)
///         .unwrap();
/// }
/// ```
#[proc_macro_attribute]
pub fn contract_bundle_provider(attr: TokenStream, item: TokenStream) -> TokenStream {
    match contract_bundle_provider_internal(attr.into(), item.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Auxiliary function to enter ?-based error propagation.
fn contract_bundle_provider_internal(
    _attr: TokenStream2,
    item: TokenStream2,
) -> SynResult<TokenStream2> {
    let enum_item = parse_bundle_enum(item)?;
    let bundle_registry = build_contracts();
    Ok(bundle_registry.generate_bundle_provision(enum_item))
}

fn parse_bundle_enum(item: TokenStream2) -> SynResult<ItemEnum> {
    let enum_item = syn::parse2::<ItemEnum>(item)?;

    if !enum_item.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            enum_item.generics.params,
            "ContractBundleProvider must not be generic",
        ));
    }
    if !enum_item.variants.is_empty() {
        return Err(syn::Error::new_spanned(
            enum_item.variants,
            "ContractBundleProvider must not have variants",
        ));
    }

    Ok(enum_item)
}
