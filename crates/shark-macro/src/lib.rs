use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Ident, Token};

struct IdentifierArray {
    identifiers: Punctuated<Ident, Token![,]>,
}

impl Parse for IdentifierArray {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let identifiers = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(IdentifierArray { identifiers })
    }
}

#[proc_macro]
pub fn make_keywords(input: TokenStream) -> TokenStream {
    let IdentifierArray {
        identifiers: keywords,
    } = parse_macro_input!(input as IdentifierArray);

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

    let expanded = quote! {
        #keyword_enum
        #mapping_function
    };

    TokenStream::from(expanded)
}
