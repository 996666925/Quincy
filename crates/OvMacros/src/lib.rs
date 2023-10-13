use proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Comp)]
pub fn comp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let name_str = name.to_string();

    let expanded = quote! {

        use crate::ecs::component::{ComponentTrait,Named,Updated,V8};
        #[typetag::serde]
        impl ComponentTrait for #name {
            fn getName(&self) -> &str {
                Self::typeName()
            }
        }
        impl Named for #name{
            fn typeName() -> &'static str {
                #name_str
            }
        }
        impl Updated for #name{

        }
        impl V8 for #name{

        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let name_str = name.to_string();

    let expanded = quote! {

        use OvCore::ecs::component::{ComponentTrait,Named};
        #[typetag::serde]
        impl ComponentTrait for #name {
            fn getName(&self) -> &str {
                &self.name
            }
        }
        impl Named for #name{
            fn typeName() -> &'static str {
                #name_str
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
