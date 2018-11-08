use syn::{Ident, Variant, VariantData};

pub fn impl_enum_default_iter(name: &Ident, variants: &[Variant]) -> quote::Tokens {
    let interface = quote::Ident::from(format!("_EnumDefaultValueIterator{}", name));
    let size = variants.len();

    let default_values: Vec<quote::Tokens> = default_values(&name, variants)
        .iter()
        .map(|i| {
            let mut toks = quote::Tokens::new();
            let x = i.as_str().to_owned().replace("<", ":: <");
            for f in x.split_whitespace() {
                toks.append(f);
            }
            toks
        })
        .collect();

    let default_values_by_name: Vec<quote::Tokens> = default_values_by_name(&name, variants)
        .iter()
        .map(|i| {
            let mut toks = quote::Tokens::new();
            let x = i.as_str().to_owned().replace("<", ":: <");
            for f in x.split_whitespace() {
                toks.append(f);
            }
            toks
        })
        .collect();

    quote! {
        #[derive(Debug, Default)]
        pub struct #interface {
            count: usize,
        }

        impl #name {
            fn enum_default_value_iter() -> #interface {
                #interface::default()
            }
        }

        impl #interface {
            fn from_usize(n: usize) -> #name {
                match n {
                    #(#default_values)*
                    _ => unreachable!(), // I think
                }
            }

            fn default_from_name(&self, name: &str) -> Option<Option<#name>> {
                match name {
                    #(#default_values_by_name)*
                    _ => None,
                }
            }
        }

        impl ::std::iter::Iterator for #interface {
            type Item = #name;
            fn next(&mut self) -> Option<Self::Item> {
                if self.count >= #size { return None }
                let result = #interface::from_usize(self.count);
                self.count += 1;
                Some(result)
            }
        }
    }
}

fn default_values(name: &Ident, variants: &[Variant]) -> Vec<quote::Tokens> {
    let mut result = Vec::new();

    for (idx, variant) in variants.iter().enumerate() {
        let id = &variant.ident;
        let new = match variant.data {
            VariantData::Unit => quote! {
                #idx => #name::#id,
            },

            VariantData::Tuple(ref fields) => {
                let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
                quote! {
                    #idx => #name::#id( #(#types::default(),)* ),
                }
            }

            VariantData::Struct(ref fields) => {
                let items: Vec<_> = fields
                    .iter()
                    .map(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;

                        quote! {
                            #ident: #ty::default()
                        }
                    })
                    .collect();

                quote! {
                    #idx => #name::#id { #(#items,)*  },
                }
            }
        };
        result.push(new);
    }
    result
}

fn default_values_by_name(name: &Ident, variants: &[Variant]) -> Vec<quote::Tokens> {
    let mut result = Vec::new();

    for (_idx, variant) in variants.iter().enumerate() {
        let id = &variant.ident;
        let key = &variant.ident.to_string();

        let new = match variant.data {
            VariantData::Unit => quote! {
                #key => Some(Some(#name::#id)),
            },

            VariantData::Tuple(ref fields) => {
                let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
                quote! {
                    #key => Some(Some(#name::#id( #(#types::default(),)* ))),
                }
            }

            VariantData::Struct(ref fields) => {
                let items: Vec<_> = fields
                    .iter()
                    .map(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;

                        quote! {
                            #ident: #ty::default()
                        }
                    })
                    .collect();

                quote! {
                    #key => Some(Some(#name::#id { #(#items,)*  })),
                }
            }
        };
        result.push(new);
    }
    result
}
