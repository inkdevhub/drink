//! Procedural macro providing a `#[drink::test]` attribute for `drink`-based contract testing.

#![warn(missing_docs)]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use crate::{codegen::generate_code, ir::IR};

mod codegen;
mod ir;

type SynResult<T> = Result<T, syn::Error>;

/// Defines a drink!-based test.
///
/// # Requirements
///
/// - `drink` crate should be available in the target crate's dependencies (at the path `::drink`).
/// - You mustn't import `drink::test` in the scope, where the macro is used. In other words, you
/// should always use the macro only with a qualified path `#[drink::test]`.
///
/// # Impact
///
/// This macro will take care of building all needed contracts for the test. The building process
/// will be executed during runtime. This means that the test will take longer to execute if the
/// contracts are not already built.
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
///
/// # Macro configuration
///
/// ## Build mode
///
/// You can specify whether the contracts should be built in debug or release mode. By default,
/// the contracts will be built in release mode. To change this, add the following to the macro
/// usage:
/// ```rust, ignore
/// #[drink::test(compile_in_debug_mode)]
/// ```
///
/// ## Manifests (contracts to be built)
///
/// You can specify, which contracts should be built for the test. *By default, `drink` will assume
/// that the current crate is the only contract to be built.* To change this, you can specify
/// all manifests that should be built for the test (also the current crate if this is the case).
/// The manifests are specified as paths relative to the current crate's root. For example:
/// ```rust, ignore
/// #[drink::test(
///     manifest = "./Cargo.toml",
///     manifest = "../second-contract/Cargo.toml",
///     manifest = "../third-contract/Cargo.toml",
/// )]
/// ```
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    match test_internal(attr.into(), item.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Auxiliary function to enter ?-based error propagation.
fn test_internal(attr: TokenStream2, item: TokenStream2) -> SynResult<TokenStream2> {
    let ir = IR::try_from((attr, item))?;
    generate_code(ir)
}
