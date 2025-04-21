use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, Item, ItemFn, ItemStruct, Lit, Meta, parse_macro_input};

use std::fs;

/// A proc macro that imports documentation from an external markdown file.
///
/// # Example Usage
///
/// ```rust
/// use external_doc::external_doc;
///
/// #[external_doc(path = "docs/my_function.md", key = "Function")]
/// pub fn my_function() {
///     // Function implementation
/// }
/// ```
///
/// In the Markdown file "docs/my_function.md":
///
/// ```markdown
/// # Function
/// This is documentation for my_function.
/// It will be used as the documentation for the function.
///
/// # Another Section
/// This section would not be pulled in by the above example.
/// ```
// Define our own structure for parsing the attribute arguments
struct ExternalDocArgs {
    args: Punctuated<Meta, Comma>,
}

impl Parse for ExternalDocArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ExternalDocArgs {
            args: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro_attribute]
pub fn external_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as ExternalDocArgs).args;
    let input = parse_macro_input!(item as Item);

    let mut doc_path = None;
    let mut doc_key = None;

    for arg in args {
        match arg {
            Meta::NameValue(nv) if nv.path.is_ident("path") => {
                if let Expr::Lit(expr_lit) = nv.value {
                    if let Lit::Str(lit_str) = expr_lit.lit {
                        doc_path = Some(lit_str.value());
                    }
                }
            }
            Meta::NameValue(nv) if nv.path.is_ident("key") => {
                if let Expr::Lit(expr_lit) = nv.value {
                    if let Lit::Str(lit_str) = expr_lit.lit {
                        doc_key = Some(lit_str.value());
                    }
                }
            }
            _ => {}
        }
    }

    let doc_path = doc_path.expect("Must specify Markdown path");
    let doc_key = doc_key.expect("Must specify key");

    let markdown = fs::read_to_string(&doc_path).expect("Failed to read .md file");

    // Parse markdown: simple header-to-section map
    let mut docs_map = std::collections::HashMap::new();
    let mut current_key = String::new();
    let mut current_lines = Vec::new();

    for line in markdown.lines() {
        if let Some(stripped) = line.strip_prefix("# ") {
            if !current_key.is_empty() {
                docs_map.insert(current_key.clone(), current_lines.join("\n"));
            }
            current_key = stripped.trim().to_string();
            current_lines = Vec::new();
        } else if !current_key.is_empty() {
            current_lines.push(line.trim_end().to_string());
        }
    }
    if !current_key.is_empty() {
        docs_map.insert(current_key.clone(), current_lines.join("\n"));
    }

    let doc_comment = docs_map
        .get(&doc_key)
        .map(String::as_str)
        .unwrap_or("No documentation found for item.");

    let doc_lines: Vec<_> = doc_comment
        .lines()
        .map(|line| quote! { #[doc = #line] })
        .collect();

    let output = match input {
        Item::Fn(item_fn) => {
            quote! {
                #(#doc_lines)*
                #item_fn
            }
        }
        Item::Struct(item_struct) => {
            quote! {
                #(#doc_lines)*
                #item_struct
            }
        }
        _ => panic!("#[external_doc] can only be applied to functions or structs"),
    };

    output.into()
}
