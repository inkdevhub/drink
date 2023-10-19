use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream as TokenStream2;
use syn::ItemFn;

#[derive(FromMeta)]
struct MacroArgs {
    #[darling(default, multiple, rename = "manifest")]
    manifests: Vec<String>,
    #[darling(default)]
    compile_in_debug_mode: bool,
}

pub struct IR {
    args: MacroArgs,
    function: ItemFn,
}

impl TryFrom<(TokenStream2, TokenStream2)> for IR {
    type Error = syn::Error;

    fn try_from((attr, item): (TokenStream2, TokenStream2)) -> Result<Self, Self::Error> {
        let args = MacroArgs::from_list(&NestedMeta::parse_meta_list(attr)?)?;
        let item_fn = syn::parse2::<ItemFn>(item)?;
        Ok(IR {
            args,
            function: item_fn,
        })
    }
}

impl IR {
    pub fn manifests(&self) -> &[String] {
        &self.args.manifests
    }

    pub fn compile_in_debug_mode(&self) -> bool {
        self.args.compile_in_debug_mode
    }

    pub fn function(&self) -> &ItemFn {
        &self.function
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    fn empty_function() -> TokenStream2 {
        quote! { fn test() {} }
    }

    #[test]
    fn argument_parsing() {
        let attr = quote! {
            manifest = "../foo/Cargo.toml",
            manifest = "some path",
            compile_in_debug_mode
        };
        let ir = IR::try_from((attr, empty_function())).expect("failed to parse macro args");

        assert_eq!(ir.manifests(), &["../foo/Cargo.toml", "some path"]);
        assert!(ir.compile_in_debug_mode());
    }

    #[test]
    fn default_arguments() {
        let attr = quote! {};
        let ir = IR::try_from((attr, empty_function())).expect("failed to parse macro args");

        assert_eq!(ir.manifests(), &Vec::<String>::new());
        assert!(!ir.compile_in_debug_mode());
    }
}
