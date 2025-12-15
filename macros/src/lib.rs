mod check;

use proc_macro::TokenStream;

#[proc_macro_derive(Check, attributes(check))]
pub fn validex(input: TokenStream) -> TokenStream {
    check::expand(&syn::parse_macro_input!(input)).into()
}
