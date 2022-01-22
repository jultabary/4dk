use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_query_handler_in_bus(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl QueryHandlerInBus for #name {
            fn handle_from_bus(&self, query: &dyn Query) -> Responses {
                self.handle_generic_query(query)
            }

            fn get_associated_query_from_bus(&self) -> TypeId {
                self.get_associated_query()
            }

            fn as_any(&self) -> &dyn Any {
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
        impl Query for #name {
            fn as_any(&self) -> &dyn Any {
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
        impl Response for #name {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
    gen.into()
}