use proc_macro2::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Error, Result};
use quote::quote;

pub fn melt(input: &DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Struct(data) => impl_struct(input, data),
        _ => Err(Error::new_spanned(input, "derive MeltRecord only support structs")),
    }
}

fn impl_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream> {
    Ok(quote!{})
}

