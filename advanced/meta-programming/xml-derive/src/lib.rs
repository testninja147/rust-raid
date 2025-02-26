use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// # Procedural Macro
/// -------------------
///
/// A procedural macro is another macro type, that accepts token stream and then
/// adds additional property to the existing data and generates code at compile
/// time.
///
/// To know more about procedural macro, please check the following:
/// https://doc.rust-lang.org/reference/procedural-macros.html#procedural-macros
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
    let field_name = fields.iter().map(|f| {
        let k = f.ident.as_ref().unwrap(); // unwrap here as we expect named fields
        quote! { #k }
    });

    let expanded = quote! {

        impl #name {
            fn to_xml(&self)-> String{
                let children = vec![
                    #(format!("<{k}>{v}</{k}>", k=stringify!(#field_name).to_string(), v=self.#field_name),)*
                ];
                return format!("<{k}>\n  {v}\n</{k}>",k=stringify!(#name),v=children.join("\n  "));
            }
        }
    };
    return TokenStream::from(expanded);
}
