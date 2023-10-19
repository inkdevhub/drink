use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream as TokenStream2;
use syn::ItemFn;

/// The macro arguments (available configuration).
#[derive(FromMeta)]
struct MacroArgs {
    /// The manifests of the contracts to be built for the test.
    #[darling(default, multiple, rename = "manifest")]
    manifests: Vec<String>,
    /// Whether the contracts should be built in the debug mode.
    #[darling(default)]
    compile_in_debug_mode: bool,
}

/// Intermediate representation of the macro arguments and configuration.
pub struct IR {
    /// Macro configuration.
    args: MacroArgs,
    /// The attributed function (testcase).
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
