#![recursion_limit = "128"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Body;

mod default_values;
mod field_names;

use default_values::*;
use field_names::*;

/// Provides an iterator over enum field names, providing a way to enumerate the fields
///
/// # Examples
/// ```
/// extern crate enum_iterator;
/// use enum_iterator::*;
///
/// #[derive(Debug, EnumNameIterator)]
/// enum Items {
///     A,
///     B,
///     C(u32)
/// }
///
/// for i in Items::enum_name_iter() {
///     println!("Field: {}", i);
/// }
/// ```
#[proc_macro_derive(EnumNameIterator)]
pub fn enum_name_iterator(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let gen = match ast.body {
        Body::Enum(ref variants) => impl_enum_name_iter(name, variants),
        Body::Struct(_) => panic!("EnumNameIterator is not implemented for structs yet!"),
    };
    gen.parse().unwrap()
}

/// Provides an iterator over enum fields default value. Also, a way to get those values by name
///
/// # Examples
/// ```
/// extern crate enum_iterator;
/// use enum_iterator::*;
///
/// #[derive(Debug, EnumDefaultValueIterator)]
/// pub enum Items {
///     A,
///     B,
///     C(u32)
/// }
///
/// for i in Items::enum_default_value_iter() {
///     println!("Default: {:?}", i);
/// }
/// ```
#[proc_macro_derive(EnumDefaultValueIterator)]
pub fn enum_default_value_iterator(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let gen = match ast.body {
        Body::Enum(ref variants) => impl_enum_default_iter(name, variants),
        Body::Struct(_) => panic!("EnumDefaultValueIterator is not implemented for structs yet!"),
    };
    gen.parse().unwrap()
}
