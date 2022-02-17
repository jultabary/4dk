use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_policy_handler_in_bus(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl dddk_core::dddk::external_event::policy_handler::PolicyHandlerInBus for #name {
            fn handle_from_bus(&self, external_event: &dyn dddk_core::dddk::external_event::external_event::ExternalEvent) -> dddk_core::dddk::aliases::Commands {
                self.handle_generic_external_event(external_event)
            }

            fn get_associated_external_event_from_bus(&self) -> std::any::TypeId {
                self.get_associated_external_event()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn get_policy_handler_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}

pub fn impl_external_event(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl dddk_core::dddk::external_event::external_event::ExternalEvent for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn get_external_event_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}