use syn::{Ident, Variant};

pub fn impl_enum_name_iter(name: &Ident, variants: &[Variant]) -> quote::Tokens {
    let interface = quote::Ident::from(format!("_EnumItemNameIterator{}", name));
    let match_usize = variant_names(&name, variants);
    let size = variants.len();

    quote! {
        #[derive(Debug, Default)]
        pub struct #interface {
            count: usize,
        }

        impl #name {
            fn enum_name_iter() -> #interface {
                #interface::default()
            }
        }

        impl #interface {
            fn from_usize(n: usize) -> String {
                match n {
                    #(#match_usize)*
                    _ => unreachable!(), // I think
                }
            }
        }

        impl ::std::iter::Iterator for #interface {
            type Item = String;
            fn next(&mut self) -> Option<Self::Item> {
                if self.count >= #size { return None }
                let result = format!("{}", #interface::from_usize(self.count));
                self.count += 1;
                Some(result.to_owned())
            }
        }
    }
}

fn variant_names(_name: &Ident, variants: &[Variant]) -> Vec<quote::Tokens> {
    let mut result = Vec::new();
    for (idx, variant) in variants.iter().enumerate() {
        let id = &variant.ident.to_string();
        let new = match variant.data {
            _ => quote! {
                #idx => #id.to_owned(),
            },
        };
        result.push(new);
    }
    result
}
