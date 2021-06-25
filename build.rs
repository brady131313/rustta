use std::{env, fs, path::PathBuf};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustta_bindgen::meta::{
    func_info::FuncInfo,
    params::{
        input::{Input, InputType},
        opt_input::OptInputType,
    },
    Meta,
};

fn main() {
    let meta = Meta::new().unwrap();
    let indicator_modules = generate_indicator_modules(&meta);
    let code = indicator_modules.to_string();
    println!("{}", code);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_path.join("indicators.rs"), code).expect("Coundn't write indicators");
}

fn generate_indicator_modules(meta: &Meta) -> TokenStream {
    let mut group_modules: Vec<TokenStream> = Vec::new();

    for (group, indicators) in meta.data.iter() {
        let func_structs = indicators
            .iter()
            .map(generate_func_struct)
            .collect::<Vec<_>>();

        let group_ident = format_ident!("{}", rustify_name(group));
        group_modules.push(quote! {
            pub mod #group_ident {
                use std::{ffi::CString, error::Error, convert::TryFrom};
                use rustta_bindgen::meta::func_handle::FuncHandle;

                #(#func_structs)*
            }
        })
    }

    quote! {
        #(#group_modules)*
    }
}

fn generate_func_struct(indicator: &FuncInfo) -> TokenStream {
    let indicator_ident = format_ident!("{}", indicator.camel_case_name());
    let indicator_id = indicator.name();
    let indicator_members = generate_indicator_struct_members(indicator);
    let indicator_doc = format!("{}", indicator.hint());
    let indicator_calculate_func = generate_indicator_calculate_func(indicator);

    quote! {
        #[doc = #indicator_doc]
        #[derive(Builder, Debug, PartialEq, Clone, Copy)]
        pub struct #indicator_ident {
            #(#indicator_members),*
        }

        impl #indicator_ident {
            const ID: &'static str = #indicator_id;

            #indicator_calculate_func
        }
    }
}

fn generate_indicator_struct_members(indicator: &FuncInfo) -> Vec<TokenStream> {
    let mut params = Vec::new();

    for param in indicator.params() {
        let member_ident = format_ident!("{}", rustify_name(param.display_name()));

        let member_type = match param.param_type() {
            OptInputType::Integer => "i32",
            OptInputType::Real => "f32",
        };
        let member_type_ident = format_ident!("{}", member_type);

        let member_default = match param.param_type() {
            OptInputType::Integer => format!("{}", param.default() as i32),
            OptInputType::Real => format!("{:.1}", param.default()),
        };

        let member_doc = format!("{}", param.hint());

        params.push(quote! {
            #[doc = #member_doc]
            #[builder(default = #member_default)]
            pub #member_ident: #member_type_ident
        })
    }

    params
}

fn generate_indicator_calculate_func(indicator: &FuncInfo) -> TokenStream {
    let (func_input_bounds, func_inputs) = generate_calculate_inputs(indicator);

    quote! {
        pub fn calculate<#(#func_input_bounds),*>(&self, #(#func_inputs),*) -> Result<(), Box<dyn Error>> {
            let handle = FuncHandle::try_from(Self::ID)?;
            Ok(())
        }
    }
}

fn generate_calculate_inputs(indicator: &FuncInfo) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let letters = 'A'..'Z';
    let mut inputs = Vec::new();
    let mut bounds = Vec::new();

    for (input, letter) in indicator.inputs().iter().zip(letters) {
        let input_ident = format_ident!("{}", rustify_input(input.name()));
        let input_bound = format_ident!("{}", letter);
        let input_type = generate_input_type(input);

        inputs.push(quote! { #input_ident: #input_bound });
        bounds.push(quote! { #input_bound: AsRef<#input_type> });
    }

    (bounds, inputs)
}

fn generate_input_type(input: &Input) -> TokenStream {
    match input.param_type() {
        InputType::Integer => quote! { [i32] },
        InputType::Real => quote! { [f32] },
        InputType::Price => quote! { [f32] },
    }
}

fn rustify_name(name: &str) -> String {
    name.clone()
        .to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
}

fn rustify_input(name: &str) -> String {
    name.clone()
        .to_lowercase()
        .replace("in", "")
        .replace("price", "price_")
}
