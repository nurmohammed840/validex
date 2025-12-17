use quote2::proc_macro2::{TokenStream, TokenTree};
use quote2::{Quote, quote};
use syn::*;

pub fn expand(input: &DeriveInput) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = input;

    let body = quote(|t| match data {
        Data::Struct(DataStruct { fields, .. }) => {
            for field in fields {
                if let (Some(input), Some(key)) = (get_validex_field(field), &field.ident) {
                    let name = key.to_string();
                    split_comma(input, |input| {
                        quote!(t, { ::validex::__field(#name, &#input, &self.#key)?; });
                    });
                }
            }
        }
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut t = TokenStream::new();
    quote!(t, {
        impl #impl_generics #ident #ty_generics #where_clause {
            fn check(&self) -> ::std::result::Result<(), ::validex::errors::FieldError<'_>> {
                #body
                ::std::result::Result::Ok(())
            }
        }
    });
    t
}

pub fn get_validex_field(field: &Field) -> Option<&TokenStream> {
    field.attrs.iter().find_map(|attr| match &attr.meta {
        Meta::List(kv) => kv.path.is_ident("check").then_some(&kv.tokens),
        _ => None,
    })
}

fn split_comma(tokens: &TokenStream, mut f: impl FnMut(TokenStream)) {
    let tokens = tokens.clone().into_iter();
    let mut split = TokenStream::new();
    for tt in tokens {
        match tt {
            TokenTree::Punct(p) if p.as_char() == ',' => {
                if !split.is_empty() {
                    f(std::mem::take(&mut split));
                }
            }
            tt => split.extend([tt]),
        }
    }
    if !split.is_empty() {
        f(split);
    }
}
