use std::collections::HashMap;

use quote::{Tokens, ToTokens};
use syn::{Body, Field, Ident, Variant, VariantData, MacroInput, Ty};
use utils::{numbered_vars, number_idents};


/// Provides the hook to expand `#[derive(From)]` into an implementation of `From`
pub fn expand(input: &MacroInput, _: &str) -> Tokens {
    let input_type = &input.ident;
    match input.body {
        Body::Struct(VariantData::Tuple(ref fields)) => {
            if fields.len() == 1 {
                newtype_from(input_type, &fields[0].ty)
            } else {
                tuple_from(input_type, fields)
            }
        }
        Body::Struct(VariantData::Struct(ref fields)) => {
            if fields.len() == 1 {
                newtype_struct_from(input_type, &fields[0])
            } else {
                struct_from(input_type, fields)
            }
        }
        Body::Enum(ref variants) => enum_from(input_type, variants),
        _ => panic!("Only tuple structs and enums can derive From"),
    }
}

fn newtype_from(input_type: &Ident, original_type: &Ty) -> Tokens {
    quote!{
        impl ::std::convert::From<#original_type> for #input_type {
            fn from(original: #original_type) -> #input_type {
                #input_type(original)
            }
        }
    }
}

fn newtype_struct_from(input_type: &Ident, field: &Field) -> Tokens {
    let field_name = &field.ident;
    let field_ty = &field.ty;
    quote!{
        impl ::std::convert::From<#field_ty> for #input_type {
            fn from(original: #field_ty) -> #input_type {
                #input_type{#field_name: original}
            }
        }
    }
}


fn tuple_from<T: ToTokens>(input_type: &T, fields: &Vec<Field>) -> Tokens {
    let field_names = &number_idents(fields.len());
    let types: &Vec<_> = &fields.iter().map(|f| &f.ty).collect();
    quote!{
        impl ::std::convert::From<(#(#types),*)> for #input_type {
            fn from(original: (#(#types),*)) -> #input_type {
                #input_type(#(original.#field_names),*)
            }
        }
    }
}

fn struct_from<T: ToTokens>(input_type: &T, fields: &Vec<Field>) -> Tokens {
    let argument_field_names = &number_idents(fields.len());
    let types: &Vec<_> = &fields.iter().map(|f| &f.ty).collect();
    let field_names: &Vec<_> = &fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    quote!{
        impl ::std::convert::From<(#(#types),*)> for #input_type {
            fn from(original: (#(#types),*)) -> #input_type {
                #input_type{#(#field_names: original.#argument_field_names),*}
            }
        }
    }
}

fn enum_from(enum_ident: &Ident, variants: &Vec<Variant>) -> Tokens {
    let mut types = vec![];
    let mut idents = vec![];
    let mut type_counts = HashMap::new();

    for variant in variants {
        match variant.data {
            VariantData::Tuple(ref structs) => {
                if structs.len() == 1 {
                    let ty = &structs[0].ty;
                    idents.push(&variant.ident);
                    types.push(ty);
                    let counter = type_counts.entry(ty).or_insert(0);
                    *counter += 1;
                }
            }
            _ => {}
        }
    }

    let mut tokens = Tokens::new();

    for (ident, old_type) in idents.iter().zip(types) {
        if *type_counts.get(&old_type).unwrap() != 1 {
            // If more than one newtype is present don't add automatic From, since it is
            // ambiguous.
            continue;
        }

        tokens.append(&quote!(
            impl ::std::convert::From<#old_type> for #enum_ident {
                fn from(original: #old_type) -> #enum_ident {
                    #enum_ident::#ident(original)
                }
            }
        )
            .to_string())
    }
    tokens
}
