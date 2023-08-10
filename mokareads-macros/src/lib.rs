extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnumVariants)]
pub fn enum_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    // Extract the variant names
    let variants = if let syn::Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        panic!("EnumVariants macro can only be used with enums");
    };

    let variant_names = variants.iter().map(|variant| &variant.ident);

    let gen = quote! {
        impl #enum_name {
            pub fn all_variants() -> Vec<#enum_name> {
                vec![ #( #enum_name::#variant_names ),* ]
            }
        }
    };

    gen.into()
}
