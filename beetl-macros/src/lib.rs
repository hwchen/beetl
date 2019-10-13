extern crate proc_macro;

mod attr;
mod expand;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(MeltRecord, attributes(melt))]
pub fn derive_melt(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::melt(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

