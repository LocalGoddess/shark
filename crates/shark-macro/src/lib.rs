use crate::util::IdentifierArray;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod util;

#[proc_macro]
pub fn make_keywords(input: TokenStream) -> TokenStream {
    let IdentifierArray {
        identifiers: keywords,
    } = parse_macro_input!(input as IdentifierArray);

    // Start - Enum Creation
    let keyword_variants = keywords.iter().map(|x| {
        quote! {
            #x
        }
    });
    let keyword_enum = quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum KeywordKind {
            #(#keyword_variants),*
        }
    };
    // End - Enum Creation

    // Start - Mapping Function
    let mapping_arms = keywords.iter().map(|x| {
        let identifier_string = x.to_string().to_lowercase();
        quote! {
            #identifier_string => Some(KeywordKind::#x),
        }
    });

    let mapping_function = quote! {
        impl KeywordKind {
            pub fn create_keyword(identifier: &str) -> Option<Self> {
                match identifier {
                    #(#mapping_arms)*
                    _ => None,
                }
            }
        }
    };
    // End - Mapping Function

    let expanded = quote! {
        #keyword_enum
        #mapping_function
    };

    TokenStream::from(expanded)
}
