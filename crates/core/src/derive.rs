use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::DeriveInput;
use syn::ExprClosure;
use syn::Ident;
use syn::LitStr;
use syn::PathSegment;

fn get_inner_type(segment: &PathSegment) -> Option<&syn::Type> {
    if let syn::PathArguments::AngleBracketed(ref args) = segment.arguments {
        if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
            return Some(inner_type);
        }
    }
    None
}

pub fn impl_derive_deserialize(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;

    let data_struct = match input.data {
        syn::Data::Struct(data_struct) => data_struct,
        _ => panic!("only works on structs"),
    };

    let field_idents = data_struct
        .fields
        .iter()
        .map(|field| {
            let ident = field.ident.clone();
            quote! { #ident, }
        })
        .collect::<TokenStream>();

    let field_impls = data_struct
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.ident.clone().expect("Couldn't get field name");
            let field_name_lit = field_name.to_string();
            // de_hypertext(selector = "")
            let mut selector: Option<LitStr> = None;
            // de_hypertext(attribute = "")
            let mut attribute: Option<LitStr> = None;
            // de_hypertext(trim)
            let mut trim = false;
            // de_hypertext(transform)
            let mut transform: Option<ExprClosure> = None;

            // parsing all attributes inside de_hypertext
            field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("de_hypertext"))
                .map(|macro_attribute| {
                    macro_attribute.parse_nested_meta(|meta| {
                        if meta.path.is_ident("selector") {
                            selector = Some(meta.value()?.parse()?);
                            return Ok(());
                        }

                        if meta.path.is_ident("trim") {
                            trim = true;
                            return Ok(());
                        }

                        if meta.path.is_ident("attribute") {
                            attribute = Some(meta.value()?.parse()?);
                            return Ok(());
                        }

                        if meta.path.is_ident("transform") {
                            transform = Some(meta.value()?.parse()?);
                            return Ok(());
                        }

                        Err(meta.error(format!(
                            "unrecognized de_hypertext attribute '{}'",
                            meta.path.clone().into_token_stream().to_string()
                        )))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            
            let meta_attributes = MetaAttributes {
                selector: selector.clone(),
                attribute: attribute.clone(),
                transform: transform.clone(),
            };


            let (let_selector_impl, select_impl) = match &selector {
                Some(selector) => (
                    quote! {
                        let selector = &de_hypertext::scraper::Selector::parse(#selector).map_err(|_| {
                            de_hypertext::DeserializeError::BuildingSelectorFailed {
                                struct_name: std::any::type_name::<Self>().to_string(),
                                field: #field_name_lit.to_string(),
                                selector: #selector.to_string(),
                            }
                        })?;
                    },
                    quote! {
                        .select(
                            &de_hypertext::scraper::Selector::parse(#selector).map_err(|_| {
                                de_hypertext::DeserializeError::BuildingSelectorFailed {
                                    struct_name: std::any::type_name::<Self>().to_string(),
                                    field: #field_name_lit.to_string(),
                                    selector: #selector.to_string(),
                                }
                            })?
                        )
                        .next()
                        .ok_or(de_hypertext::DeserializeError::ElementNotFoud {
                            struct_name: std::any::type_name::<Self>().to_string(),
                            field: #field_name_lit.to_string(),
                            selector: #selector.to_string(),
                        })
                    },
                ),
                None => (quote!{}, quote! {}),
            };

            match &field.ty {
                syn::Type::Path(type_path) => {
                    if let Some(segment) = type_path.path.segments.first() {
                        if segment.ident.to_string() == "Vec" {
                            if let Some(inner_type) = get_inner_type(segment) {
                                return quote! {
                                    let #field_name = {
                                        document
                                            .select(
                                                &de_hypertext::scraper::Selector::parse(#selector).map_err(|_| {
                                                    de_hypertext::DeserializeError::BuildingSelectorFailed {
                                                        struct_name: std::any::type_name::<Self>().to_string(),
                                                        field: #field_name_lit.to_string(),
                                                        selector: #selector.to_string(),
                                                    }
                                                })?
                                            )
                                            .map(|document| #inner_type::from_document(&document))
                                            .collect::<Result<Vec<#inner_type>, _>>()?
                                    };
                                }
                            }
                        }

                        if segment.ident.to_string() == "Option" {
                            if let Some(inner_type) = get_inner_type(segment) {
                                return match inner_type.to_token_stream().to_string().as_str() {
                                    "String" => impl_field_option_string(field_name, meta_attributes),
                                    _ => quote! {
                                        let #field_name = {
                                            #let_selector_impl
                                            match document.select(&selector).next() {
                                                Some(document) => match #inner_type::from_document(&document) {
                                                    Ok(#field_name) => Some(#field_name),
                                                    Err(_) => None
                                                },
                                                None => None,
                                            }
                                        };
                                    },
                                }
                            }
                        }
                    }

                    if type_path.path.is_ident("String") {
                        return impl_field_string(field_name, meta_attributes);
                    }

                    quote! {
                        let #field_name = {
                            let document = document
                                #select_impl?;
                            #type_path::from_document(&document)?
                        };
                    }
                },
                _ => panic!("unsupported type"),
            }
        })
        .collect::<TokenStream>();

    quote!(
        impl de_hypertext::Deserializer for #struct_name {
            fn from_document(document: &de_hypertext::scraper::ElementRef) -> Result<Self, de_hypertext::DeserializeError> {
                #field_impls
                Ok(Self { #field_idents })
            }
        }
    )
}

// The idea to use a struct here is to specifically get compiler warnings when deconstructing it,
// forcing us to think about the newly added meta attribute for each type case.
struct MetaAttributes {
    pub selector: Option<LitStr>,
    pub attribute: Option<LitStr>,
    pub transform: Option<ExprClosure>,
}

fn impl_selector(field_name: &Ident, selector: &LitStr) -> TokenStream {
    let field_name = field_name.to_string();
    quote! {
        de_hypertext::scraper::Selector::parse(#selector).map_err(|_| {
            de_hypertext::DeserializeError::BuildingSelectorFailed {
                struct_name: std::any::type_name::<Self>().to_string(),
                field: #field_name.to_string(),
                selector: #selector.to_string(),
            }
        })
    }
}

fn impl_field_string(field_name: &Ident, meta_attributes: MetaAttributes) -> TokenStream {
    let MetaAttributes {
        selector,
        attribute,
        transform,
    } = meta_attributes;
    let string_impl = match (selector, attribute, transform) {
        (None, None, None) => quote! {
            document.text().collect::<String>()
        },
        (None, None, Some(transform)) => quote! {
            (#transform)(document.text().collect::<String>())
        },
        (None, Some(attribute), None) => {
            let field_name = field_name.to_string();
            quote! {
                document
                    .value()
                    .attr(#attribute)
                    .map(|s| s.to_string())
                    .ok_or(de_hypertext::DeserializeError::AttributeNotFound {
                        struct_name: std::any::type_name::<Self>().to_string(),
                        field: #field_name.to_string(),
                        selector: None,
                        attribute: #attribute.to_string(),
                    })?
            }
        },
        (None, Some(attribute), Some(transform)) => quote! {
            (#transform)(
                document
                    .value()
                    .attr(#attribute)
                    .map(|s| s.to_string())
                    .ok_or(de_hypertext::DeserializeError::AttributeNotFound {
                        struct_name: std::any::type_name::<Self>().to_string(),
                        field: #field_name.to_string(),
                        selector: None,
                        attribute: #attribute
                    })?
            )
        },
        (Some(selector), None, None) => {
            let selector_impl = impl_selector(&field_name, &selector);
            let field_name = field_name.to_string();
            quote! {
                document
                    .select(&#selector_impl?)
                    .next()
                    .ok_or(de_hypertext::DeserializeError::ElementNotFoud {
                        struct_name: std::any::type_name::<Self>().to_string(),
                        field: #field_name.to_string(),
                        selector: #selector.to_string(),
                    })?
                    .text()
                    .collect::<String>()
            }
        },
        (Some(selector), None, Some(transform)) => {
            let selector_impl = impl_selector(&field_name, &selector);
            let field_name = field_name.to_string();
            quote! {
                (#transform)(
                    document
                        .select(&#selector_impl?)
                        .next()
                        .ok_or(de_hypertext::DeserializeError::ElementNotFoud {
                            struct_name: std::any::type_name::<Self>().to_string(),
                            field: #field_name.to_string(),
                            selector: #selector.to_string(),
                        })?
                        .text()
                        .collect::<String>()
                )
            }
        },
        (Some(selector), Some(attribute), None) => {
            let selector_impl = impl_selector(&field_name, &selector);
            let field_name = field_name.to_string();
            quote! {
                document
                    .select(&#selector_impl?)
                    .next()
                    .ok_or(de_hypertext::DeserializeError::ElementNotFoud {
                        struct_name: std::any::type_name::<Self>().to_string(),
                        field: #field_name.to_string(),
                        selector: #selector.to_string(),
                    })?
                    .value()
                    .attr(#attribute)
                    .map(|s| s.to_string())
                    .ok_or(de_hypertext::DeserializeError::AttributeNotFound {
                        struct_name: std::any::type_name::<Self>().to_string(),
                        field: #field_name.to_string(),
                        selector: Some(#selector.to_string()),
                        attribute: #attribute.to_string(),
                    })?
            }
        },
        (Some(selector), Some(attribute), Some(transform)) => {
            let selector_impl = impl_selector(&field_name, &selector);
            let field_name = field_name.to_string();
            quote! {
                (#transform)(
                    document
                        .select(&#selector_impl?)
                        .next()
                        .ok_or(de_hypertext::DeserializeError::ElementNotFoud {
                            struct_name: std::any::type_name::<Self>().to_string(),
                            field: #field_name.to_string(),
                            selector: #selector.to_string(),
                        })?
                        .value()
                        .attr(#attribute)
                        .map(|s| s.to_string())
                        .ok_or(de_hypertext::DeserializeError::AttributeNotFound {
                            struct_name: std::any::type_name::<Self>().to_string(),
                            field: #field_name.to_string(),
                            selector: Some(#selector.to_string()),
                            attribute: #attribute.to_string(),
                        })?
                )
            }
        },
    };

    quote! {
        let #field_name = #string_impl;
    }
} 

fn impl_field_option_string(field_name: &Ident, meta_attributes: MetaAttributes) -> TokenStream {
    let MetaAttributes {
        selector,
        attribute,
        transform,
    } = meta_attributes;
    let option_string_impl = match (selector, attribute, transform) {
        (None, None, None) => quote! {
            Some(document.text().collect::<String>())
        },
        (None, None, Some(transform)) => {
            quote! {
                Some((#transform)(document.text().collect::<String>()))
            }
        }
        (None, Some(attribute), None) => quote! {
            document.value().attr(#attribute)
        },
        (None, Some(attribute), Some(transform)) => quote! {
            document
                .value()
                .attr(#attribute)
                .map(|s| s.to_string())
                .map(#transform);
        },
        (Some(selector), None, None) => {
            let impl_selector = impl_selector(&field_name, &selector);
            quote! {
                document
                    .select(&#impl_selector?)
                    .next()
                    .map(|document| document.text().collect::<String>())
            }
        }
        (Some(selector), None, Some(transform)) => {
            let impl_selector = impl_selector(&field_name, &selector);
            quote! {
                document
                    .select(&#impl_selector?)
                    .next()
                    .map(|document| document.text().collect::<String>())
                    .map(#transform)
            }
        }
        (Some(selector), Some(attribute), None) => {
            let impl_selector = impl_selector(&field_name, &selector);
            quote! {
                document
                    .select(&#impl_selector?)
                    .next()
                    .map(|document| document.value().attr(#attribute))
                    .flatten()
                    .map(|s| s.to_string())
            }
        }
        (Some(selector), Some(attribute), Some(transform)) => {
            let impl_selector = impl_selector(&field_name, &selector);
            quote! {
                document
                    .select(&#impl_selector?)
                    .next()
                    .map(|document| document.value().attr(#attribute))
                    .flatten()
                    .map(|s| s.to_string())
                    .map(#transform)
            }
        }
    };

    quote! {
        let #field_name = #option_string_impl;
    }
}
