use proc_macro2::TokenStream;
use quote::quote;

use crate::{ir::IR, SynResult};

pub fn generate_code(ir: IR) -> SynResult<TokenStream> {
    let test_function = ir.function();

    Ok(quote! {
        #[test]
        #test_function
    })
}
