use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, ReturnType, parse_macro_input, punctuated::Punctuated, token::Comma};

use func_struct::{FuncMetadata, FuncParam};

fn get_params(inputs: &Punctuated<FnArg, Comma>) -> Vec<FuncParam> {
    let mut params = Vec::new();

    for param in inputs {
        if let FnArg::Typed(pat_ty) = param {
            let param_name = &pat_ty.pat;
            let param_type = &pat_ty.ty;

            params.push(FuncParam {
                name: quote! {#param_name}.to_string(),
                r#type: quote! {#param_type}.to_string(),
            })
        }
    }

    params
}

#[proc_macro_attribute]
pub fn func_metadata(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident.to_string();
    let fn_name_upper = format_ident!("{}_JSON", fn_name.to_uppercase());

    let params = get_params(&input.sig.inputs);

    let output = match &input.sig.output {
        ReturnType::Default => "None".to_string(),
        ReturnType::Type(_, ty) => quote! {#ty}.to_string(),
    };

    let fn_metadata = FuncMetadata {
        name: fn_name.clone(),
        input: params,
        output: output,
    };

    let fn_metadata_string = serde_json::to_string(&fn_metadata).unwrap();
    let fn_metadata_str = fn_metadata_string.as_str();

    quote! {
        #input
        const #fn_name_upper: &str = #fn_metadata_str;
    }
    .into()
}
