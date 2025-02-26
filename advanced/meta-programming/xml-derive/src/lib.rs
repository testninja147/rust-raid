use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Serialize)]
pub fn serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // list fields of the structure.
    let fields = &match input.data {
        syn::Data::Struct(data_struct) => &data_struct.fields.clone(),
        _ => panic!("Only Struct is supported"),
    };

    // Collect the names of the struct's fields
    let field_names = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap(); // unwrap here as we expect named fields
        quote! { stringify!(#field_name) }
    });

    let expanded = quote! {

        impl #name {
            fn serialize(&self)-> String{
                let keys = vec![
                    #(format!("<{k}>{v}</{k}>", k=#field_names.to_string(), v="CONTENT"),)*
                ];
                return format!("<{k}>{v}</{k}>",k=stringify!(#name),v=keys.join(""));
            }
        }
    };
    return TokenStream::from(expanded);
}
