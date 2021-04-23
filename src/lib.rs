//! Derive basic error type infrastruture for enum types.
//!
//! Supports unnamed and unit enum variants, and uses the type definition
//!  to derive `std::fmt::Display` and `std::error:Error` for the error type,
//!  as well as `std::from::From<T>` for any unnamed variant with one parameter
//!  inferred to be an error type (currently determined by whether it's type
//!  name is Error).
//!
//! Default behaviour can be overridden with the auto_error attribute
//!  - format_str takes a string which becomes the format string for that
//!    variant
//!  - make_from forces derivation of std::from::From when set to true
//!  - err forces the std::error::Error implementation to return the inner
//!    type during calls to source, or in other words to treat the inner
//!    type as an error type.
//!
//! From derivation and source returning work only for variants with a single field.
//!
//! # Example
//!
//! ```
//! #[derive(AutoError)]
//! use autoerror::AutoError;
//!
//! enum Error {
//!     #[auto_error(format_str="Document not found")]
//!     NotFound,
//!     IO(std::io::Error),
//!     #[auto_error(make_from=true)]
//!     Other(String),
//! }
//! ```

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::{quote, format_ident};

// Infer whether wrapped type is an error
//  by applying a name based heuristic (type path
//  last segment is Error)
fn infer_is_error(variant: &syn::Variant) -> bool {
    if let syn::Fields::Named(_) = variant.fields {
        return false;
    }

    if variant.fields.len() != 1 {
        return false;
    }
    let field = variant.fields.iter().next().unwrap();

    if let syn::Type::Path(path) = &field.ty {
        if path.path.segments.len() == 0 {
            return false;
        }
        if path.path.segments.last().unwrap().ident == "Error" {
            return true;
        }
    }

    false
}

// Auto-generate a basic format string for a variant.
fn infer_format_str(variant: &syn::Variant) -> String {
    let mut result = "".to_string();
    let mut first = true;
    for _var in variant.fields.iter() {
        if first {
            result += "{}";
            first = false;
        } else {
            result += " {}";
        }
    }
    result
}

struct ErrorVariant<'a> {
    err: bool,
    make_from: bool,
    format_str: String,
    variant: &'a syn::Variant,
}

// Parse a single variant in the enum
fn parse_variant(variant: &syn::Variant) -> Result<ErrorVariant, TokenStream> {
    // validate fields are unnamed (but present!)
    match variant.fields {
        syn::Fields::Named(_) => { return Err(TokenStream::from(syn::Error::new_spanned(variant, "Named fields not supported").to_compile_error())); }
        syn::Fields::Unnamed(_) => {}
        syn::Fields::Unit => { }
    }

    let mut attr: Option<_> = None;
    for attr_cand in variant.attrs.iter() {
        if attr_cand.path.is_ident("auto_error") {
            if attr != None {
                return Err(TokenStream::from(syn::Error::new_spanned(&attr_cand, "Duplicate occurence of auto_error attribute").to_compile_error()));
            }
            attr = Some(attr_cand);
        }
    }

    let mut result = ErrorVariant {
        err: infer_is_error(variant),
        make_from: infer_is_error(variant),
        format_str: infer_format_str(variant),
        variant,
    };

    if let Some(attr) = attr {
        let meta = attr.parse_meta().map_err(|e| e.to_compile_error())?;
        let meta = match meta {
            syn::Meta::List(list) => list,
            _ => { return Err(TokenStream::from(syn::Error::new_spanned(&meta, "Incorrect auto_error arguments").to_compile_error())); },
        };

        for arg in meta.nested.iter() {
            let arg = match arg {
                syn::NestedMeta::Meta(arg) => arg,
                _ => { return Err(TokenStream::from(syn::Error::new_spanned(arg, "Incorrect auto_error arguments").to_compile_error())); },
            };
            let arg = match arg {
                syn::Meta::NameValue(arg) => arg,
                _ => { return Err(TokenStream::from(syn::Error::new_spanned(arg, "Incorrect auto_error arguments").to_compile_error())); },
            };
            if arg.path.is_ident("err") {
                result.err = match &arg.lit {
                    syn::Lit::Bool(v) => v.value,
                    _ => { return Err(TokenStream::from(syn::Error::new_spanned(&arg.lit, "Incorrect value for err, expected bool").to_compile_error())); },
                };
            } else if arg.path.is_ident("format_str") {
                result.format_str = match &arg.lit {
                    syn::Lit::Str(v) => v.value(),
                    _ => { return Err(TokenStream::from(syn::Error::new_spanned(&arg.lit, "Incorrect value for format_str, expected string").to_compile_error())); },
                };
            } else if arg.path.is_ident("make_from") {
                result.make_from = match &arg.lit {
                    syn::Lit::Bool(v) => v.value,
                    _ => { return Err(TokenStream::from(syn::Error::new_spanned(&arg.lit, "Incorrect value for make_from, expected bool").to_compile_error())); },
                };
            } else {
                return Err(TokenStream::from(syn::Error::new_spanned(variant, "Unknown parameter").to_compile_error()));
            }
        }
    }

    if result.err && result.variant.fields.len() != 1 {
        return Err(TokenStream::from(syn::Error::new_spanned(variant, "Wrapped errors should have exactly 1 argument").to_compile_error()));
    }

    if result.make_from && result.variant.fields.len() != 1 {
        return Err(TokenStream::from(syn::Error::new_spanned(variant, "Can only derive from for variants with 1 field").to_compile_error()));
    }

    Ok(result)
}

/// Derive basic error type infrastruture for enum types.
///
/// Supports unnamed and unit enum variants, and uses the type definition
///  to derive `std::fmt::Display` and `std::error:Error` for the error type,
///  as well as `std::from::From<T>` for any unnamed variant with one parameter
///  inferred to be an error type (currently determined by whether it's type
///  name is Error).
///
/// Default behaviour can be overridden with the auto_error attribute
///  - format_str takes a string which becomes the format string for that
///    variant
///  - make_from forces derivation of std::from::From when set to true
///  - err forces the std::error::Error implementation to return the inner
///    type during calls to source, or in other words to treat the inner
///    type as an error type.
///
/// From derivation and source returning work only for variants with a single field.
///
/// # Example
///
/// ```
/// #[derive(AutoError)]
/// use autoerror::AutoError;
///
/// enum Error {
///     #[auto_error(format_str="Document not found")]
///     NotFound,
///     IO(std::io::Error),
///     #[auto_error(make_from=true)]
///     Other(String),
/// }
/// ```
#[proc_macro_derive(AutoError, attributes(auto_error))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enumdecl = if let syn::Data::Enum(e) = input.data {
        e
    } else {
        return TokenStream::from(syn::Error::new_spanned(&input.ident, "AutoError only supports enums").to_compile_error());
    };

    let error_ident = input.ident;
    let error_variants: Result<Vec<_>, TokenStream> = enumdecl.variants.iter().map(|v| parse_variant(v)).collect();
    let error_variants = match error_variants {
        Ok(v) => v,
        Err(e) => {return e}
    };

    let from_impls = error_variants.iter().map(|var| {
        if !var.make_from {
            return None;
        }

        let sourcetype = &var.variant.fields.iter().next().unwrap().ty;
        let curvar = &var.variant.ident;

        Some(quote!{
            impl ::std::convert::From<#sourcetype> for #error_ident {
                fn from (e: #sourcetype) -> Self {
                    Self::#curvar(e)
                }
            }
        })
    });

    let display_branches = error_variants.iter().map(|var| {
        let format_str = &var.format_str;
        let curvar = &var.variant.ident;
        let params: Vec<_> = var.variant.fields.iter().enumerate().map(|(i, _field)| {
            format_ident!("f{}", i)
        }).collect();
        match var.variant.fields {
            syn::Fields::Unnamed(_) => quote!{
                Self::#curvar(#(#params),*) => f.write_fmt(format_args!(#format_str #(,#params)*)),
            },
            syn::Fields::Unit => quote!{
                Self::#curvar => f.write_fmt(format_args!(#format_str)),
            },
            _ => panic!("Internal error (AutoError)")
        }
        
    });

    let source_branches = error_variants.iter().map(|var| {
        if !var.err {
            return None;
        }
        let curvar = &var.variant.ident;
        Some(quote!{
            Self::#curvar(e) => Some(e),
        })
    });

    TokenStream::from(quote! {
        #(#from_impls)*

        impl ::std::fmt::Display for #error_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    #(#display_branches)*
                }
            }
        }

        impl ::std::error::Error for #error_ident {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    #(#source_branches)*
                    _ => None,
                }
            }
        }
    })
}
