use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    // Process only structs
    let struct_data = match &derive_input.data {
        Data::Struct(data) => data,
        _ => return derive_input.into_token_stream().into(),
    };

    let mut field_renames = Vec::new();

    // Iterate over all fields in the struct
    if let Fields::Named(fields) = &struct_data.fields {
        for field in &fields.named {
            if let Some(ident) = &field.ident {
                let field_name = ident.to_string();
                let renamed_field = field_name.replace('_', "").to_lowercase();
                field_renames.push(quote! {
                    #[serde(rename = #renamed_field)]
                });
            }
        }
    }

    // Generate the new code with added attributes
    let expanded = quote! {
        #[derive(serde::Deserialize)]
        #derive_input
        #(#field_renames)*
    };

    expanded.into()
}

/// Not part of the public API, do not use.
#[doc(hidden)]
#[proc_macro_attribute]
pub fn black_hole(_: TokenStream, _: TokenStream) -> TokenStream {
    TokenStream::new()
}
