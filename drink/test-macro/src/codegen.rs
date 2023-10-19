use proc_macro2::TokenStream;
use quote::quote;

use crate::{ir::IR, SynResult};

pub fn generate_code(ir: IR) -> SynResult<TokenStream> {
    let item_fn = ir.function();

    let fn_name = &item_fn.sig.ident;
    let fn_body = &item_fn.block;
    let fn_return_type = &item_fn.sig.output;
    let fn_vis = &item_fn.vis;
    let fn_attrs = &item_fn.attrs;

    let manifests = ir.manifests();
    let debug_mode = ir.compile_in_debug_mode();
    let build_contracts = quote! {
        ::drink::testing_utils::build_contracts(vec![ #( #manifests ),* ], #debug_mode);
    };

    Ok(quote! {
        #[test]
        #( #fn_attrs )*
        #fn_vis fn #fn_name() #fn_return_type {
            #build_contracts
            #fn_body
        }
    })
}
