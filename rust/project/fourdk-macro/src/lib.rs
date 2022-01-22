extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;


#[proc_macro_derive(CommandHandlerInBus)]
pub fn command_handler_in_bus_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_command_handler_in_bus(&ast)
}

fn impl_command_handler_in_bus(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl CommandHandlerInBus for #name {
            fn handle_from_bus(&self, command: &dyn Command) -> Events {
                self.handle_generic_command(command)
            }

            fn get_associated_command_from_bus(&self) -> TypeId {
                self.get_associated_command()
            }

            fn as_any(&self) -> &dyn Any {
                self
            }

            fn get_command_handler_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Command)]
pub fn command_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_command(&ast)
}

fn impl_command(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Command for #name {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn get_command_name(&self) -> String {
                stringify!(#name).to_string()
            }
        }
    };
    gen.into()
}