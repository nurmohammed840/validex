use quote2::proc_macro2::TokenStream;
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
                if let Some(func) = get_validator(field) {
                    let key = &field.ident;
                    quote!(t, {
                        ::validex::Validate::validate(&#func, &self.#key)?;
                    });
                }
            }
        }
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut t = TokenStream::new();
    quote!(t, {
        impl #impl_generics #ident #ty_generics #where_clause {
            fn validate(&self) -> ::validex::Result<()> {
                #body
                ::std::result::Result::Ok(())
            }
        }
    });
    t
}

pub fn get_validator(field: &Field) -> Option<&TokenStream> {
    field.attrs.iter().find_map(|attr| match &attr.meta {
        Meta::List(kv) => kv.path.is_ident("validate").then_some(&kv.tokens),
        _ => None,
    })
}
