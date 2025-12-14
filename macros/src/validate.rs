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
                if let (Some(func), Some(key)) = (get_validator(field), &field.ident) {
                    let name = key.to_string();
                    quote!(t, {
                        ::validex::__field(#name, &#func, &self.#key)?;
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
            fn validate(&self) -> ::validex::Result<(), ::validex::errors::FieldError> {
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
