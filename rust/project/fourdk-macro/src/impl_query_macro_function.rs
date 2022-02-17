use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_query_handler_in_bus(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl dddk_core::dddk::query::query_handler::QueryHandlerInBus for #name {
            fn handle_from_bus(&self, query: &dyn dddk_core::dddk::query::query::Query) -> dddk_core::dddk::aliases::ResponseFromHandler {
                self.handle_generic_query(query)
            }

            fn get_associated_query_from_bus(&self) -> std::any::TypeId {
                self.get_associated_query()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn get_query_handler_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}

pub fn impl_query(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl dddk_core::dddk::query::query::Query for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn get_query_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}

pub fn impl_response(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl dddk_core::dddk::query::response::Response for #name {
            fn as_any(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn get_response_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}