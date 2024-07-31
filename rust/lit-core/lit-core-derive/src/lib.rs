extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

use crate::derive::description::derive_description;
use crate::derive::error_code::derive_error_code;

pub(crate) mod derive;
pub(crate) mod utils;

#[proc_macro_derive(Description)]
#[proc_macro_error]
pub fn description(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    derive_description(&input)
}

#[proc_macro_derive(ErrorCode, attributes(code))]
#[proc_macro_error]
pub fn error_code(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    derive_error_code(&input)
}
