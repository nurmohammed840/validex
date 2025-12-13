mod validate;

use proc_macro::TokenStream;

#[proc_macro_derive(Validate, attributes(validate))]
pub fn validate(input: TokenStream) -> TokenStream {
    validate::expand(&syn::parse_macro_input!(input)).into()
}
