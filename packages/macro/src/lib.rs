use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// cw_custom: A procedural macro attribute that generates implementations of:
/// (1) the `cosmwasm_std::CustomMsg` trait and (2) the `From` trait for
/// converting the input type into an instance of `cosmwasm_std::CosmosMsg`.
///
/// # Example
///
/// ```
/// use nibiru_macro::cw_custom;
/// use cosmwasm_std::{CustomMsg, CosmosMsg};
/// use cosmwasm_schema::{cw_serde};
///
/// #[cw_serde]
/// #[cw_custom]
/// pub struct MyCustomMsg {
///   // struct fields...
/// }
/// ```
#[proc_macro_attribute]
pub fn cw_custom(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let gen = quote! {
        #ast

        impl CustomMsg for #name {}

        /// This implementation of the "From" trait converts instances of the #name
        /// type into `CossmosMsg` objects.
        impl From<#name> for CosmosMsg<#name> {
            fn from(original: #name) -> Self {
                CosmosMsg::Custom(original)
            }
        }
    };

    gen.into()
}
