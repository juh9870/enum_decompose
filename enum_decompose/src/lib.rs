#![doc = include_str!("../../README.md")]
use darling::ast::NestedMeta;
use darling::{FromAttributes, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote_spanned, Attribute as SynAttribute, Fields, ItemEnum, Meta,
    Token, TypePath, Visibility,
};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    /// Prefix for emitted struct names. If not present, the enum name will be used as prefix
    prefix: Option<String>,
    /// Suffix for emitted struct names
    suffix: Option<String>,
    /// Determines whenever empty enum variants are skipped, defaults to true
    skip_empty: Option<bool>,
    /// Default derives for emitted structs. If not provided, derives will be copied from the enum
    derive: Option<Punctuated<TypePath, Token![,]>>,
    /// Visibility for emitted structs. Inherits enum visibility if not specified
    vis: Option<Visibility>,
    /// Visibility for fields of emitted structs. Defaults to public
    fields_vis: Option<Visibility>,
}

#[derive(Debug, FromAttributes)]
#[darling(attributes(decompose))]
struct FieldArgs {
    /// Skips the field
    #[darling(default)]
    skip: bool,
    /// Custom name for the emitted struct
    rename: Option<Ident>,
    /// Custom derives for the emitted struct. Overwrites derives specified in
    /// the main macro
    derive: Option<Punctuated<TypePath, Token![,]>>,
    /// Custom visibility for the emitted struct
    vis: Option<Visibility>,
    /// Custom visibility for fields of the emitted struct
    fields_vis: Option<Visibility>,
}

/// Annotating enum with an `#[decompose]` attribute will convert all
/// non-unit variants into newtype variants by emitting variants as individual structs.
///
/// See module-level documentation for details
#[proc_macro_attribute]
pub fn decompose(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse attribute input
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(args) => args,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match MacroArgs::from_list(&attr_args) {
        Ok(args) => args,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let mut input = parse_macro_input!(input as ItemEnum);
    let enum_vis = &input.vis;
    let enum_ident = &input.ident;

    let prefix = args.prefix.unwrap_or_else(|| input.ident.to_string());
    let suffix = args.suffix.unwrap_or_default();

    let mut structs = vec![];
    let mut impls = vec![];

    let skip_empty = args.skip_empty.unwrap_or(true);

    let default_derives = args.derive;

    let mut enum_derives = Vec::<&SynAttribute>::new();
    // Enum derives are only used when `derive` argument was not provided
    if default_derives.is_none() {
        for attr in &input.attrs {
            if let Meta::List(l) = &attr.meta {
                if l.path.is_ident("derive") {
                    enum_derives.push(attr)
                }
            }
        }
    }

    let default_derives = default_derives.unwrap_or_default();

    // Process enum variants
    for variant in &mut input.variants {
        let field_args = match FieldArgs::from_attributes(&variant.attrs) {
            Ok(data) => data,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        };

        // Consume attributes
        variant
            .attrs
            .retain(|attr| !attr.path().is_ident("decompose"));

        if field_args.skip {
            continue;
        }

        if skip_empty && variant.fields.is_empty() {
            continue;
        }

        let variant_ident = &variant.ident;

        let struct_ident = field_args
            .rename
            .unwrap_or_else(|| format_ident!("{}{}{}", prefix, variant_ident, suffix));

        let mut fields = Fields::Unnamed(parse_quote_spanned! {variant.span()=>
            (#struct_ident)
        });

        let vis = field_args
            .vis
            .as_ref()
            .or(args.vis.as_ref())
            .unwrap_or(enum_vis);

        std::mem::swap(&mut variant.fields, &mut fields);

        let fields_vis = field_args
            .fields_vis
            .or_else(|| args.fields_vis.clone())
            .unwrap_or_else(|| Visibility::Public(Default::default()));

        // Patch visibility
        fields = match fields {
            Fields::Named(mut fields) => {
                for field in &mut fields.named {
                    field.vis = fields_vis.clone();
                }
                Fields::Named(fields)
            }
            Fields::Unnamed(mut fields) => {
                if fields.unnamed.is_empty() {
                    // Fix for stuff like `Name()` which is a valid enum variant syntax but not a valid struct syntax
                    Fields::Unit
                } else {
                    for field in &mut fields.unnamed {
                        field.vis = fields_vis.clone();
                    }
                    Fields::Unnamed(fields)
                }
            }
            unit @ Fields::Unit => unit,
        };

        // Named tokens don't have closed tokens, other kinds use `;`
        let closing_token = if !matches!(fields, Fields::Named(_)) {
            quote!(;)
        } else {
            quote!()
        };

        let derives = field_args.derive.as_ref().unwrap_or(&default_derives);
        let enum_derives = if field_args.derive.is_none() {
            quote!(#(#enum_derives)*)
        } else {
            quote!()
        };

        structs.push(quote_spanned! { variant.span()=>
            #[derive(#derives)]
            #enum_derives
            #vis struct #struct_ident #fields #closing_token
        });

        impls.push(quote_spanned! { variant.span()=>
            #[automatically_derived]
            impl core::convert::From<#struct_ident> for #enum_ident {
                fn from(item: #struct_ident) -> Self {
                    Self::#variant_ident(item)
                }
            }
        });
    }

    (quote! {
        #input

        #(#structs)*

        #(#impls)*
    })
    .into()
}
