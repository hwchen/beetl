use proc_macro2::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Error, Fields, Result};
use quote::quote;

pub fn melt(input: &DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Struct(data) => impl_struct(input, data),
        _ => Err(Error::new_spanned(input, "derive MeltRecord only support structs")),
    }
}

fn impl_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // first check that 
    let value_vars = match &data.fields {
        Fields::Named(fields) => Ok(&fields.named),
        _ => Err(Error::new_spanned(input, "derive MeltRecord only support named structs (no unit or tuple structs)")),
    };

    Ok(quote!{
        impl #impl_generics MeltRecord for #ty #ty_generics #where_clause {
        }
    })
}

