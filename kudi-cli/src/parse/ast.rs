use std::ops::Deref;

use quote::ToTokens;
use syn::{ItemImpl, Path, Token, parse::Parse, spanned::Spanned};

#[derive(Debug)]
pub struct ItemImplTrait(ItemImpl);

impl ItemImplTrait {
    pub fn sig_trait(&self) -> &(Option<Token![!]>, Path, Token![for]) {
        // Safety: validated before
        unsafe { self.0.trait_.as_ref().unwrap_unchecked() }
    }
}

impl Deref for ItemImplTrait {
    type Target = ItemImpl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parse for ItemImplTrait {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item_impl = ItemImpl::parse(input)?;

        if item_impl.trait_.is_some() {
            Ok(Self(item_impl))
        } else {
            Err(syn::Error::new(
                item_impl.span(),
                "expected using on impl trait",
            ))
        }
    }
}

impl ToTokens for ItemImplTrait {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}
