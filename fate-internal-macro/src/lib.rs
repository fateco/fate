use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let func_name = &func.sig.ident;
    let func_name_const = format_ident!("{}", func_name.to_string().to_uppercase());

    let (value_customname, const_customname) = if attr.is_empty() {
        (quote! {None}, None)
    } else {
        let str_customname = parse_macro_input!(attr as LitStr).value();
        let const_name_customname =
            format_ident!("{}_CUSTOMNAME", func_name.to_string().to_uppercase());
        (
            quote! {Some(#str_customname)},
            Some(quote! {pub const #const_name_customname: &str = #str_customname;}),
        )
    };

    TokenStream::from(quote! {
      #const_customname
      pub const #func_name_const: &str = concat!(module_path!(), "::", stringify!(#func_name));

      #func

      inventory::submit! {
          crate::handler::Handler(crate::handler::HandlerIds { auto: #func_name_const, custom: #value_customname }, |interaction, env| {
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
