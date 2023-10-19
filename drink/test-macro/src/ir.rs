use proc_macro::TokenStream;

pub struct IR {}

impl TryFrom<(TokenStream, TokenStream)> for IR {
    type Error = syn::Error;

    fn try_from((attr, item): (TokenStream, TokenStream)) -> Result<Self, Self::Error> {
        todo!()
    }
}
