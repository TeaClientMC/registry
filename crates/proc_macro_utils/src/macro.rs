extern crate proc_macro;

use inquire::{Confirm, MultiSelect, Text};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

#[proc_macro_derive(Promptable)]
pub fn promptable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields_named) = data.fields {
            fields_named.named
        } else {
            unimplemented!("Promptable only supports named fields")
        }
    } else {
        unimplemented!("Promptable only supports structs")
    };

    let mut prompt_stmts = Vec::new();
    let mut assignments = Vec::new();

    for field in fields.iter() {
        let f_name = field.ident.as_ref().unwrap();
        let f_name_str = f_name.to_string();
        let f_ty = &field.ty;

        // Detect types by simple string matching for this example
        let prompt_stmt = if let Type::Path(typepath) = f_ty {
            let segment = &typepath.path.segments.last().unwrap().ident.to_string();
            match segment.as_str() {
                "String" => {
                    quote! {
                        let #f_name = Text::new(#f_name_str).prompt()?;
                    }
                }
                "bool" => {
                    // Confirm prompt
                    quote! {
                        let #f_name = Confirm::new(#f_name_str).with_default(false).prompt()?;
                    }
                }
                "Vec" => {
                    // Assume MultiSelect with options in ctx named {field_name}_options (adjust as needed)
                    let options_field =
                        syn::Ident::new(&format!("{}_options", f_name), f_name.span());
                    quote! {
                        let #f_name = MultiSelect::new(#f_name_str, ctx.#options_field.clone()).prompt()?;
                    }
                }
                "Option" => {
                    // Optional skippable text prompt
                    quote! {
                        let #f_name = Text::new(#f_name_str).prompt_skippable()?;
                    }
                }
                _ => {
                    // Unsupported type - default value
                    quote! {
                        let #f_name = Default::default();
                    }
                }
            }
        } else {
            quote! {
                let #f_name = Default::default();
            }
        };

        prompt_stmts.push(prompt_stmt);
        assignments.push(quote! { #f_name });
    }

    let r#gen = quote! {
        impl Promptable for #name {
            fn prompt_with(ctx: &PromptCtx) -> Result<Self, inquire::error::InquireError> {
                #(#prompt_stmts)*

                Ok(Self {
                    #(#assignments),*
                })
            }
        }
    };

    r#gen.into()
}
