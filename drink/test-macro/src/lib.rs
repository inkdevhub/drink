use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use crate::{codegen::generate_code, ir::IR};

mod codegen;
mod ir;

type SynResult<T> = Result<T, syn::Error>;

#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    match test_internal(attr, item) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn test_internal(attr: TokenStream, item: TokenStream) -> SynResult<TokenStream2> {
    let ir = IR::try_from((attr, item))?;
    generate_code(ir)
}
