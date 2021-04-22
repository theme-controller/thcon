#![deny(clippy::all)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Disableable)]
pub fn disableable_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let gen = quote! {
        impl Disableable for #name {
            fn disabled(&self) -> bool {
                self.disabled == true
            }
        }
    };

    gen.into()
}

/// Creates a `Config` struct wrapping either the marked struct or an instance of
/// `thcon_trait::Disabled`.
#[proc_macro_derive(AppConfig)]
pub fn appconfig_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let gen = quote! {
        #[derive(Deserialize, Debug)]
        #[serde(transparent)]
        pub struct Config {
            #[serde(with = "either::serde_untagged")]
            inner: either::Either<#name, thcon_trait::Disabled>,
        }

        impl Config {
            /// Convenience function to access the enabled configuration variant
            fn unwrap_inner_left(&self) -> &(#name) {
                self.inner.as_ref().unwrap_left()
            }

            /// Convenience function to access the disabled configuration variant
            fn unwrap_inner_right(&self) -> &thcon_trait::Disabled {
                self.inner.as_ref().unwrap_right()
            }
        }
    };

    gen.into()
}