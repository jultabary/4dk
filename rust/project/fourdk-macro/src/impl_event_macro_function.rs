use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_event_handler_in_bus(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl dddk_core::dddk::event::event_handler::EventHandlerInBus for #name {
            fn handle_from_bus(&self, event: std::sync::Arc<dyn dddk_core::dddk::event::event::Event>) -> Result<(), dddk_core::dddk::aliases::GenericError> {
                self.handle_generic_event(event)
            }

            fn get_associated_event_from_bus(&self) -> std::any::TypeId {
                self.get_associated_event()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn get_event_handler_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}

pub fn impl_event(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl dddk_core::dddk::event::event::Event for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn get_event_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}