mod non_recursive;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RecursiveSchema)]
pub fn derive_non_recursive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    non_recursive::derive_non_recursive(&ast)
}
