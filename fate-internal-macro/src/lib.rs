use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let func_name = &func.sig.ident;
    let func_name_const = format_ident!("{}", func_name.to_string().to_uppercase());

    let (command_val, command_const) = if attr.is_empty() {
        (quote! {None}, None)
    } else {
        let command_name = parse_macro_input!(attr as LitStr).value();
        let func_name_command_const =
            format_ident!("{}_COMMAND", func_name.to_string().to_uppercase());
        (
            quote! {Some(#command_name)},
            Some(quote! {pub const #func_name_command_const: &str = #command_name;}),
        )
    };

    TokenStream::from(quote! {
      #command_const
      pub const #func_name_const: &str = concat!(module_path!(), "::", stringify!(#func_name));

      #func

      inventory::submit! {
          crate::handler::Handler(crate::handler::HandlerIds { command: #command_val, common: #func_name_const }, |interaction, env| {
              Box::pin(#func_name(interaction, env))
          })
      }
    })
}

#[proc_macro_attribute]
pub fn app_command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let func_name = &func.sig.ident;

    TokenStream::from(quote! {
      #func

      inventory::submit! {
          crate::slash::Slash(#func_name)
      }
    })
}
