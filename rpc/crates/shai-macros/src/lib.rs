//! Procedural macros for [`shai`](https://crates.io/crates/shai).
//!
//! See `#[shai::message]` on the `shai` crate re-export.

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Attribute, Item, Path, Token};

/// Applies `#[derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)]` to a struct or enum,
/// merging with an existing `#[derive(...)]` if present.
///
/// If any `#[derive(...)]` on the item includes `Debug`, inserts `#[rkyv(derive(Debug))]` immediately
/// after the merged `#[derive(::rkyv::Archive, ...)]` so the archived type is `Debug` too (unless an
/// existing `#[rkyv(...)]` already mentions `derive` and `Debug`). Order is required: `#[rkyv]` is a
/// derive helper and must follow `#[derive(Archive, ...)]`.
///
/// On a `mod`, walks all items recursively and does the same for every `struct` / `enum` (including
/// nested modules). Other items are left unchanged.
///
/// Pair with `shai::rpc! { ... }` to assign message IDs. This macro does not register IDs.
///
/// # Requirements
///
/// The defining crate must depend on `rkyv` so that `::rkyv::...` resolves. Extra rkyv attributes
/// (`#[rkyv(...)]`, `#[archive_attr(...)]`, etc.) can be placed next to `#[shai::message]` as usual.
#[proc_macro_attribute]
pub fn message(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "unexpected arguments; use `#[shai::message]` with no parameters",
        )
        .to_compile_error()
        .into();
    }

    let mut item = parse_macro_input!(item as Item);
    if let Err(e) = apply_top_level(&mut item) {
        return e.to_compile_error().into();
    }
    quote!(#item).into()
}

fn apply_top_level(item: &mut Item) -> syn::Result<()> {
    match item {
        Item::Struct(s) => {
            add_rkyv_derives(&mut s.attrs);
        }
        Item::Enum(e) => {
            add_rkyv_derives(&mut e.attrs);
        }
        Item::Mod(m) => apply_mod_contents(m),
        other => {
            return Err(syn::Error::new(
                other.span(),
                "#[shai::message] can only be applied to struct, enum, or mod",
            ));
        }
    }
    Ok(())
}

fn apply_mod_contents(m: &mut syn::ItemMod) {
    let Some((_, items)) = &mut m.content else {
        return;
    };
    for item in items.iter_mut() {
        match item {
            Item::Struct(s) => add_rkyv_derives(&mut s.attrs),
            Item::Enum(e) => add_rkyv_derives(&mut e.attrs),
            Item::Mod(m) => apply_mod_contents(m),
            _ => {}
        }
    }
}

fn path_last_ident(path: &Path) -> Option<&syn::Ident> {
    path.segments.last().map(|s| &s.ident)
}

fn paths_have_ident(paths: &Punctuated<Path, Token![,]>, want: &str) -> bool {
    paths
        .iter()
        .any(|p| path_last_ident(p).is_some_and(|i| i == want))
}

fn any_derive_has_debug(attrs: &[Attribute]) -> bool {
    attrs.iter().filter(|a| a.path().is_ident("derive")).any(|attr| {
        attr
            .parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
            .map(|p| paths_have_ident(&p, "Debug"))
            .unwrap_or(false)
    })
}

fn rkyv_attrs_already_derive_debug(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("rkyv") {
            return false;
        }
        let s = attr.meta.to_token_stream().to_string();
        s.contains("derive") && s.contains("Debug")
    })
}

fn add_rkyv_derives(attrs: &mut Vec<Attribute>) {
    const NAMES: &[&str] = &["Archive", "Serialize", "Deserialize"];

    let want_archived_debug =
        any_derive_has_debug(attrs) && !rkyv_attrs_already_derive_debug(attrs);

    let mut derive_index = None;
    let mut paths: Punctuated<Path, Token![,]> = Punctuated::new();

    for (i, attr) in attrs.iter().enumerate() {
        if attr.path().is_ident("derive") {
            derive_index = Some(i);
            if let Ok(p) = attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated) {
                paths = p;
            }
            break;
        }
    }

    for name in NAMES {
        if !paths_have_ident(&paths, name) {
            let path: Path = match *name {
                "Archive" => parse_quote!(::rkyv::Archive),
                "Serialize" => parse_quote!(::rkyv::Serialize),
                "Deserialize" => parse_quote!(::rkyv::Deserialize),
                _ => unreachable!(),
            };
            paths.push(path);
        }
    }

    let new_attr: Attribute = parse_quote!(#[derive(#paths)]);

    match derive_index {
        Some(i) => attrs[i] = new_attr,
        None => {
            attrs.insert(0, new_attr);
        }
    }

    if want_archived_debug {
        let insert_at = derive_index.map(|i| i + 1).unwrap_or(1);
        attrs.insert(insert_at, parse_quote!(#[rkyv(derive(Debug))]));
    }
}
