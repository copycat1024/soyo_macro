extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Fields, Index};

#[proc_macro_derive(View)]
pub fn derive_view(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = syn::parse(input).unwrap();

    let view = quote! {
        View
    };

    let render_code = gen_render_code(&data);

    quote! {
        impl #generics #view for #ident #generics {
            fn render(&self) {
                #render_code
            }
        }
    }
    .into()
}

fn gen_render_code(data: &Data) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(data) => gen_field_render_code(&data.named),
            Fields::Unnamed(data) => gen_field_render_code(&data.unnamed),
            _ => quote!(),
        },
        Data::Enum(_) => quote!(),
        _ => quote!(),
    }
}

fn gen_field_render_code(data: &Punctuated<Field, Comma>) -> proc_macro2::TokenStream {
    let data = data
        .iter()
        .enumerate()
        .map(|(i, field)| match &field.ident {
            Some(ident) => quote!(#ident),
            None => {
                let i = Index::from(i);
                quote!(#i)
            }
        })
        .map(|field| {
            quote! {
                <dyn View>::render(&self.#field);
            }
        });

    quote! {
        #(#data)*
    }
}
