use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_event_handler_in_bus(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl EventHandlerInBus for #name {
            fn handle_from_bus(&self, event: Arc<dyn Event>) {
                self.handle_generic_event(event)
            }

            fn get_associated_event_from_bus(&self) -> TypeId {
                self.get_associated_event()
            }

            fn as_any(&self) -> &dyn Any {
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
        impl Event for #name {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn get_event_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}