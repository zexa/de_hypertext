use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::LitStr;

#[proc_macro_derive(Deserialize, attributes(de_hypertext))]
pub fn derive_deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

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

                        Err(meta.error(format!(
                            "unrecognized de_hypertext attribute {:?}",
                            meta.path
                        )))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            let trim = match trim {
                true => quote! {
                    .trim()
                },
                false => quote! {},
            };

            let selector = selector.expect("missing #[de_hypertext(selector)]");

            match &field.ty {
                syn::Type::Path(type_path) => {
                    if let Some(segment) = type_path.path.segments.first() {
                        if segment.ident.to_string() == "Vec" {
                            if let syn::PathArguments::AngleBracketed(ref args) = segment.arguments {
                                if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                                    return quote! {
                                        let #field_name = {
                                            let selector = scraper::Selector::parse(#selector)?;
                                            document
                                                .select(&selector)
                                                .into_iter()
                                                .map(|document| #inner_type::from_document(&document))
                                                .collect::<Result<Vec<#inner_type>, _>>()?
                                        };
                                    }
                                }
                            }
                        }

                        if segment.ident.to_string() == "Option" {
                            if let syn::PathArguments::AngleBracketed(ref args) = segment.arguments {
                                if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                                    return quote! {
                                        let #field_name = {
                                            let selector = scraper::Selector::parse(#selector)?;
                                            match document.select(&selector).next() {
                                                Some(document) => match #inner_type::from_document(&document) {
                                                    Ok(#field_name) => Some(#field_name),
                                                    Err(_) => None
                                                },
                                                None => None,
                                            }
                                        };
                                    }
                                }
                            }
                        }
                    }

                    if type_path.path.is_ident("String") {
                        let text_or_attr = match attribute {
                            Some(attribute) => quote! {
                                .value()
                                .attr(#attribute)
                                .ok_or(de_hypertext::DeserializeError::AttributeNotFound {
                                    struct_name: std::any::type_name::<#struct_name>().to_string(),
                                    field: #field_name_lit.to_string(),
                                    selector: #selector.to_string(),
                                    attribute: #attribute.to_string(),
                                })?
                                #trim
                                .to_string()
                            },
                            None => quote! {
                                .text()
                                .collect::<String>()
                                #trim
                                .to_string()
                            },
                        };

                        return quote! {
                            let #field_name = {
                                let selector = scraper::Selector::parse(#selector)?;
                                document
                                    .select(&selector)
                                    .next()
                                    .ok_or(de_hypertext::DeserializeError::ElementNotFoud {
                                        struct_name: std::any::type_name::<#struct_name>().to_string(),
                                        field: #field_name_lit.to_string(),
                                        selector: #selector.to_string(),
                                    })?
                                    #text_or_attr
                            };
                        }
                    }

                    quote! {
                        let #field_name = {
                            let selector = scraper::Selector::parse(#selector)?;
                            let document = document
                                .select(&selector)
                                .next()
                                .ok_or(de_hypertext::DeserializeError::ElementNotFoud {
                                    struct_name: std::any::type_name::<#struct_name>().to_string(),
                                    field: #field_name_lit.to_string(),
                                    selector: #selector.to_string(),
                                })?;
                            #type_path::from_document(&document)?
                        };
                    }
                },
                _ => panic!("unsupported type"),
            }
        })
        .collect::<TokenStream>();

    quote!(
        impl de_hypertext::Deserializer<Self> for #struct_name {
            fn from_document(document: &scraper::ElementRef) -> Result<Self, Box<dyn std::error::Error>> {
                #field_impls
                Ok(Self { #field_idents })
            }
        }
    )
    .into()
}
