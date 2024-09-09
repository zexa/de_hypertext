use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::DeriveInput;
use syn::ExprClosure;
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
            let field_name = &field.ident;
            let field_name_lit = field.ident.as_ref().map(|ident| ident.to_string());
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

            let transform = match transform {
                Some(transform) => quote! {
                    let transform = #transform;
                    let value = transform(value);
                },
                None => quote! {},
            };

            let (selector_opt, select_impl) = match &selector {
                Some(selector) => (
                    quote! {
                        Some(#selector.to_string())
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
                None => (quote! {None}, quote! {}),
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
                                    "String" => {
                                        match attribute {
                                            Some(attribute) => {
                                                let attribute_impl = match selector.is_some() {
                                                    true => quote!{
                                                        .ok()
                                                        .map(|document|
                                                            document
                                                                .value()
                                                                .attr(#attribute)
                                                                .map(|attribute| attribute.trim().to_string())
                                                        )
                                                        .flatten()
                                                    },
                                                    false => quote!{
                                                        .value()
                                                        .attr(#attribute)
                                                        .map(|attribute| attribute.trim().to_string())
                                                    },
                                                };

                                                quote! {
                                                    let #field_name = {
                                                        let value = document
                                                            #select_impl
                                                            #attribute_impl
                                                        ;
                                                        #transform
                                                        value
                                                    };
                                                }
                                            },
                                            None => quote! {
                                                let #field_name = {
                                                    let value = document
                                                        #select_impl
                                                        .ok()
                                                        .map(|document| document.text().collect::<String>())
                                                    ;
                                                    #transform
                                                    value
                                                };
                                            }
                                        }
                                    },
                                    _ => quote! {
                                        let #field_name = {
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
                        let text_or_attr_impl = match attribute {
                            Some(attribute) => quote! {
                                .value()
                                .attr(#attribute)
                                .ok_or(de_hypertext::DeserializeError::AttributeNotFound {
                                    struct_name: std::any::type_name::<Self>().to_string(),
                                    field: #field_name_lit.to_string(),
                                    selector: #selector_opt,
                                    attribute: #attribute.to_string(),
                                })?
                                .to_string()
                            },
                            None => quote! {
                                .text()
                                .collect::<String>()
                                .to_string()
                            },
                        };

                        let q = match selector.is_some() {
                            true => quote!{?},
                            false => quote!{},
                        };

                        return quote! {
                            let #field_name = {
                                let value = document
                                    #select_impl
                                    #q
                                    #text_or_attr_impl
                                ;
                                #transform
                                value
                            };
                        }
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
