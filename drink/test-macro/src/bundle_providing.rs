use std::{collections::HashMap, path::PathBuf};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::ItemEnum;

pub struct BundleProviderGenerator {
    root_contract_name: Option<String>,
    bundles: HashMap<String, PathBuf>,
}

impl BundleProviderGenerator {
    pub fn new<I: Iterator<Item = (String, PathBuf)>>(
        bundles: I,
        root_contract_name: Option<String>,
    ) -> Self {
        let root_contract_name = root_contract_name.map(|name| name.to_case(Case::Pascal));
        let bundles = HashMap::from_iter(bundles.map(|(name, path)| {
            let name = name.to_case(Case::Pascal);
            (name, path)
        }));

        if let Some(root_contract_name) = &root_contract_name {
            assert!(
                bundles.contains_key(root_contract_name),
                "Root contract must be part of the bundles"
            );
        }

        Self {
            root_contract_name,
            bundles,
        }
    }

    pub fn generate_bundle_providing(&self, enum_item: ItemEnum) -> TokenStream2 {
        let enum_name = &enum_item.ident;
        let enum_vis = &enum_item.vis;
        let enum_attrs = &enum_item.attrs;

        let local = match &self.root_contract_name {
            None => quote! {},
            Some(root_name) => {
                let local_bundle = self.bundles[root_name].to_str().expect("Invalid path");
                quote! {
                    pub fn local() -> ::drink::ContractBundle {
                        ::drink::ContractBundle::from_path(#local_bundle)
                    }
                }
            }
        };

        let bundle_matches = self.bundles.iter().map(|(name, path)| {
            let path = path.to_str().expect("Invalid path");
            quote! {
                #enum_name::#name => ::drink::ContractBundle::from_path(#path),
            }
        });

        let contract_names = self.bundles.keys();

        quote! {
            #(#enum_attrs)*
            #[derive(Copy, Clone, PartialEq, Eq, Debug)]
            #enum_vis enum #enum_name {
                #(#contract_names,)*
            }

            impl #enum_name {
                #local

                pub fn bundle(self) -> ::drink::ContractBundle {
                    match self {
                        #(#bundle_matches,)*
                    }
                }
            }
        }
    }
}
