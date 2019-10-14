// add a transform fn for var (col name)? or just a rename on the field?
// add ability to rename the value and vars fields
// check that all value vars have the same type
//
// currently, value vars are moved, so the type needs to impl Copy. Could check the type and use
// clone() if not Copy.

use crate::attr;
use proc_macro2::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Error, Field, Fields, Result};
use quote::{format_ident, quote};

pub fn melt(input: &DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Struct(data) => impl_struct(input, data),
        _ => Err(Error::new_spanned(input, "derive MeltRecord only supports structs")),
    }
}

fn impl_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream> {
    let ty = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let iterator_ty = format_ident!("{}Melt", ty);
    let output_ty = format_ident!("{}Output", iterator_ty);

    // first find value_vars
    let (id_vars, value_vars) = match &data.fields {
        Fields::Named(fields) => get_id_value_vars_members(&fields.named)?,
        _ => return Err(Error::new_spanned(input, "derive MeltRecord only supports named structs (no unit or tuple structs)")),
    };

    let id_vars_idents = id_vars.iter().map(|field| &field.ident);
    let id_vars_fields = id_vars.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;
        quote!(#ident: #ty)
    });

    let value_ty = value_vars.iter().map(|field| field.ty.clone()).nth(0).expect("there should be at least one value var");
    let value_vars_idents: Vec<_> = value_vars.iter()
        .map(|field| field.ident.clone().expect("deal with this later; only named structs allowed"))
        .collect();

    let value_vars_len = value_vars.len();

    let var_match = value_vars_idents.iter().enumerate()
        .map(|(idx, var)| {
            let var = var.to_string();
            quote!(#idx => #var.into())
        });

    let value_match = value_vars_idents.iter().enumerate()
        .map(|(idx, value)| {
            quote!(#idx => self.row.#value)
        });

    Ok(quote!{
        impl #impl_generics MeltRecord<#iterator_ty #ty_generics, #output_ty #ty_generics> for #ty #ty_generics #where_clause {
            fn melt(self) -> #iterator_ty #ty_generics {
                #iterator_ty {
                    row: self,
                    count: 0usize,
                }
            }
        }

        struct #iterator_ty #ty_generics #where_clause {
            row: #ty #ty_generics,
            count: usize,
        }
        impl #impl_generics Iterator for #iterator_ty #ty_generics #where_clause {
            type Item = #output_ty #ty_generics;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count < #value_vars_len {
                    let var = match self.count {
                        #(#var_match,)*
                        _ => return None,
                    };
                    let value = match self.count {
                        #(#value_match,)*
                        _ => return None,
                    };
                    let res = #output_ty {
                        #(#id_vars_idents: self.row.#id_vars_idents,)*
                        var,
                        value,
                    };

                    self.count += 1;

                    Some(res)
                } else {
                    None
                }
            }
        }

        #[derive(Debug)]
        struct #output_ty #ty_generics #where_clause {
            #(#id_vars_fields,)*
            var: String,
            value: #value_ty,
        }
    })
}

fn get_id_value_vars_members<'a>(fields: impl IntoIterator<Item = &'a Field>) -> Result<(Vec<Field>, Vec<Field>)> {
    let mut id_vars = Vec::new();
    let mut value_vars = Vec::new();

    for field in fields.into_iter() {
        if attr::is_value_var(field)? {
            value_vars.push(field.clone());
        } else {
            id_vars.push(field.clone());
        }
    }

    Ok((id_vars, value_vars))
}

