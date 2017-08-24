#![feature(custom_attribute)]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Body, DeriveInput, Ident, VariantData, MetaItem, NestedMetaItem};

#[proc_macro_derive(CreateTablePostgres)]
pub fn create_table_postgres(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_create_table_postgres(&ast);
    gen.parse().unwrap()
}

fn impl_create_table_postgres(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    quote! {
        impl CreateTablePostgres for #name {
            fn create_table(&self) -> String{
                "Create Postgres Table".to_owned()
            }
        }
    }
}

#[proc_macro_derive(Melt, attributes(melt))]
pub fn melt(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_melt(&ast);
    gen.parse().unwrap()
}

fn impl_melt(ast: &DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut tys = Vec::new();

    let idents: Vec<Ident> = match ast.body {
        Body::Struct(ref vdata) => {
            match vdata {
                &VariantData::Struct(ref fields) => {
                    let mut idents = Vec::new();
                    for field in fields.iter() {
                        if let Some(attr) = field.attrs.get(0) {
                            if let MetaItem::List(ref i, ref items) = attr.value {
                                if i.to_string() == "melt" {
                                    if items.len() != 1 {
                                        panic!("Only one item allowed in melt attr");
                                    }
                                    if let Some(item) = items.get(0) {
                                        if let &NestedMetaItem::MetaItem(ref mitem) = item {
                                            if let &MetaItem::Word(ref w) = mitem {
                                                if w.to_string() != "value_var" {
                                                    panic!("only \"value_var\" allowed as an attr value");
                                                }
                                                match field.ident {
                                                    Some(ref ident) => idents.push(ident.clone()),
                                                    None => panic!("Your struct is missing a field identity."),
                                                }
                                                tys.push(field.ty.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    idents
                },
                &VariantData::Tuple(_) | &VariantData::Unit => {
                    panic!("Only derivable for structs with fields");
                },
            }
        },
        Body::Enum(_) => panic!("Only derivable for structs, not enums"),
    };

    //TODO check all tys
    let val_type = tys.get(0).ok_or("No types found").unwrap();

    // Eventually ues generics for target, to allow many targets
    // for now just get one
    let targets: Vec<_> = {
        let mut targets = Vec::new();
        for attr in &ast.attrs {
            match attr.value {
                MetaItem::List(ref i, ref metaitems) => {
                    if i.to_string() == "melt_targets" {
                        // now find targets
                        for m in metaitems {
                            if let &NestedMetaItem::MetaItem(ref m) = m {
                                if let &MetaItem::Word(ref w) = m {
                                    targets.push(w);
                                }
                            }
                        }
                    }
                },
                _ => panic!("To derive Melt, must have #[melt(<target>)] attr."),

            }
        }
        targets
    };
    println!("{:?}", targets);
    let target = targets.get(0).expect("no target found");

    let mut idents_str = Vec::new();
        for ident in idents.iter() {
                idents_str.push(String::from(ident.as_ref()));
        };

    quote! {
        impl Melt for #name {
            type V = #val_type;
            type T = #target;

            fn melt<F>(&mut self, out: &mut Vec<Self::T>, f: &F) where F: Fn(&str, Self::V, &Self, &mut Vec<Self::T>) {
                #(
                    let val = self.#idents;

                    f(#idents_str, val, &self, out);


                )*
            }
        }
    }
}
