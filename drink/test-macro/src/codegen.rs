use proc_macro2::TokenStream;
use quote::quote;

use crate::{ir::IR, SynResult};

/// Interpret the intermediate representation and generate the final code.
pub fn generate_code(ir: IR) -> SynResult<TokenStream> {
    let item_fn = ir.function();

    // Unfortunately, we have to extract all these items, because we are going to change the body
    // a bit.
    let fn_signature = &item_fn.sig;
    let fn_body = &item_fn.block;
    let fn_vis = &item_fn.vis;
    let fn_attrs = &item_fn.attrs;

    // Prepare the code responsible for building the contracts.
    let _manifests = ir.manifests();
    let _debug_mode = ir.compile_in_debug_mode();
    let build_contracts = quote! {};

    Ok(quote! {
        #[test]
        #( #fn_attrs )*
        #fn_vis #fn_signature {
            #build_contracts
            #fn_body
        }
    })
}
