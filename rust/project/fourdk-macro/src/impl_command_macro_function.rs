use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_command_handler_in_bus(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl dddk_core::dddk::command::command_handler::CommandHandlerInBus for #name {

            fn handle_from_bus(&self, command: &dyn dddk_core::dddk::command::command::Command) -> dddk_core::dddk::aliases::Events {
                self.handle_generic_command(command)
            }

            fn get_associated_command_from_bus(&self) -> std::any::TypeId {
                self.get_associated_command()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn get_command_handler_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}

pub fn impl_command(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl dddk_core::dddk::command::command::Command for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn get_command_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}
