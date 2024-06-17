use syn::{parse::Parse, punctuated::Punctuated, Ident, Token};

pub struct IdentifierArray {
    pub identifiers: Punctuated<Ident, Token![,]>,
}

impl Parse for IdentifierArray {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let identifiers = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(IdentifierArray { identifiers })
    }
}
