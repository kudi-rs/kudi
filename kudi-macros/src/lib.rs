mod target;
mod utils;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use target::{stateful_target, stateless_target};

#[proc_macro_derive(DepInj, attributes(target))]
pub fn dep_inj(input: TokenStream) -> TokenStream {
    stateful_target(parse_macro_input!(input)).into()
}

#[proc_macro_attribute]
pub fn target(_attr: TokenStream, item: TokenStream) -> TokenStream {
    stateless_target(parse_macro_input!(item)).into()
}
