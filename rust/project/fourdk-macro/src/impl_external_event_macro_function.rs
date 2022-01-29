use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_policy_handler_in_bus(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl PolicyHandlerInBus for #name {
            fn handle_from_bus(&self, external_event: &dyn ExternalEvent) -> Commands {
                self.handle_generic_external_event(external_event)
            }

            fn get_associated_external_event_from_bus(&self) -> TypeId {
                self.get_associated_external_event()
            }

            fn as_any(&self) -> &dyn Any {
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
        impl ExternalEvent for #name {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn get_external_event_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}