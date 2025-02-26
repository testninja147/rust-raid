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
    let tags = fields.iter().map(|f| {
        let k = f.ident.as_ref().unwrap(); // unwrap here as we expect named fields
        quote! { #k }
    });

    let expanded = quote! {

        impl #name {
            fn serialize(&self)-> String{
                let children = vec![
                    #(format!("<{k}>{v}</{k}>", k=stringify!(#tags).to_string(), v=self.#tags),)*
                ];
                return format!("<{k}>{v}</{k}>",k=stringify!(#name),v=children.join(""));
            }
        }
    };
    return TokenStream::from(expanded);
}
