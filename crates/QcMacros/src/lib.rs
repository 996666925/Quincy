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

#[proc_macro_derive(UiComp)]
pub fn uiComp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let name_str = name.to_string();

    let expanded = quote! {

        use QcCore::ecs::component::{ComponentTrait,Named,Updated,V8};
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

#[proc_macro_derive(Control)]
pub fn uiNode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let name_str = name.to_string();

    let expanded = quote! {

        use QcCore::ecs::component::{BaseComponentTrait,Named};
        use std::any::Any;
        use thunderdome::Index;
        use crate::component::{SetId,Widget};
        use std::ops::{Deref, DerefMut};
        impl BaseComponentTrait for #name {
            fn asAny(&self) -> &dyn Any {
                self
            }
            fn asAnyMut(&mut self) -> &mut dyn Any {
                self
            }
        }
        impl Named for #name{
            fn typeName() -> &'static str {
                #name_str
            }
        }

        impl SetId for #name{
            fn setId(&mut self, id: Index){
                self.id=id;
            }
        }

        
        impl Deref for #name {
            type Target = Widget;
                            
            fn deref(&self) -> &Self::Target {
                &self.widget
            }
        }
        
        impl DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.widget
            }
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

        use QcCore::ecs::component::{ComponentTrait,Named};
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
