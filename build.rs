use std::{env, fs, path::PathBuf};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustta_bindgen::meta::{
    func_info::FuncInfo,
    params::{
        input::{Input, InputFlags, InputType},
        opt_input::OptInputType,
        output::{Output, OutputType},
    },
    Meta,
};

fn main() {
    let meta = Meta::new().unwrap();
    let indicator_modules = generate_indicator_modules(&meta);
    let code = indicator_modules.to_string();

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
                #![allow(unused_imports)]

                use std::{ffi::CString, error::Error, convert::TryFrom};
                use rustta_bindgen::meta::func_handle::FuncHandle;
                use rustta_bindgen::meta::params::param_holder::{
                    ParamHolder, OptInputParam, InputParam, OutputParam, Ohlc, wrap_output
                };
                use crate::input::{Open, Low, High, Close, Volume, Length};

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
            OptInputType::Real => "f64",
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
    let opt_input_params = generate_opt_input_params(indicator);
    let input_params = generate_input_params(indicator);
    let input_length = generate_input_length(indicator);
    let func_outputs = generate_calculate_outputs(indicator);
    let output_params = generate_output_params(indicator);
    let call_and_return = generate_call_and_return(indicator);

    quote! {
        pub fn calculate<#(#func_input_bounds),*>(&self, #(#func_inputs),*) -> Result<#func_outputs, Box<dyn Error>> {
            let handle = FuncHandle::try_from(Self::ID)?;
            let mut params = ParamHolder::try_from(handle)?;

            #(#opt_input_params)*
            #(#input_params)*

            #input_length
            let output_size = params.required_output_size(0, input_len)
                .ok_or("Failed to get required size")?;

            #(#output_params)*

            #call_and_return
        }
    }
}

fn generate_opt_input_params(indicator: &FuncInfo) -> Vec<TokenStream> {
    let mut params = Vec::new();

    for param in indicator.params() {
        let position = param.position();
        let param_ident = format_ident!("{}", rustify_name(param.display_name()));

        let param_type_ident = match param.param_type() {
            OptInputType::Real => quote! { Real },
            OptInputType::Integer => quote! { Integer },
        };

        params.push(quote! {
            params.set_param(#position, OptInputParam::#param_type_ident(self.#param_ident))?;
        });
    }

    params
}

fn generate_input_params(indicator: &FuncInfo) -> Vec<TokenStream> {
    let mut inputs = Vec::new();

    for input in indicator.inputs() {
        let position = input.position();
        let input_ident = format_ident!("{}", rustify_input(input.name()));

        let input_value = match input.param_type() {
            InputType::Integer | InputType::Real => quote! { #input_ident.as_ref().as_ptr() },
            InputType::Price => {
                let flags = input.flags();

                let open = if flags.contains(InputFlags::OPEN) {
                    quote! { #input_ident.open().as_ptr() }
                } else {
                    quote! { std::ptr::null() }
                };
                let low = if flags.contains(InputFlags::LOW) {
                    quote! { #input_ident.low().as_ptr() }
                } else {
                    quote! { std::ptr::null() }
                };
                let high = if flags.contains(InputFlags::HIGH) {
                    quote! { #input_ident.high().as_ptr() }
                } else {
                    quote! { std::ptr::null() }
                };
                let close = if flags.contains(InputFlags::CLOSE) {
                    quote! { #input_ident.close().as_ptr() }
                } else {
                    quote! { std::ptr::null() }
                };
                let volume = if flags.contains(InputFlags::VOLUME) {
                    quote! { #input_ident.volume().as_ptr() }
                } else {
                    quote! { std::ptr::null() }
                };
                let open_interest = if flags.contains(InputFlags::OPEN_INTEREST) {
                    quote! { #input_ident.open_interest() }
                } else {
                    quote! { std::ptr::null()  }
                };

                quote! {
                    Ohlc {
                        open: #open,
                        low: #low,
                        high: #high,
                        close: #close,
                        volume: #volume,
                        openinterest: #open_interest
                    }
                }
            }
        };

        let input_type_ident = match input.param_type() {
            InputType::Real => quote! { Real },
            InputType::Integer => quote! { Integer },
            InputType::Price => quote! { Ohlc },
        };

        inputs.push(quote! {
            params.set_input(#position, InputParam::#input_type_ident(#input_value))?;
        })
    }

    inputs
}

fn generate_output_params(indicator: &FuncInfo) -> Vec<TokenStream> {
    let mut outputs = Vec::new();

    for output in indicator.outputs() {
        let position = output.position();
        let output_ident = format_ident!("{}", rustify_input(output.name()));

        let (output_type_ident, output_vec_init) = match output.param_type() {
            OutputType::Real => (quote! { Real }, quote! { 0.0 }),
            OutputType::Integer => (quote! { Integer }, quote! { 0 }),
        };

        outputs.push(quote! {
            let #output_ident = vec![#output_vec_init; output_size];
            let mut #output_ident = std::mem::ManuallyDrop::new(#output_ident);

            params.set_output(#position, OutputParam::#output_type_ident(#output_ident.as_mut_ptr()))?;
        })
    }

    outputs
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
        bounds.push(quote! { #input_bound: Length + #input_type });
    }

    (bounds, inputs)
}

fn generate_calculate_outputs(indicator: &FuncInfo) -> TokenStream {
    let mut outputs = Vec::new();

    for output in indicator.outputs() {
        let output_type = generate_output_type(output);

        outputs.push(output_type)
    }

    // More than one output, wrap in tuple
    if outputs.len() > 1 {
        quote! { (#(#outputs),*) }
    } else {
        quote! { #(#outputs)* }
    }
}

fn generate_input_length(indicator: &FuncInfo) -> TokenStream {
    let mut inputs = Vec::new();

    for input in indicator.inputs() {
        let input_ident = format_ident!("{}", rustify_input(input.name()));
        inputs.push(input_ident);
    }

    quote! {
        let input_len = max!(#(#inputs.length()),*) as i32;
        let end_idx = (min!(#(#inputs.length()),*) - 1) as i32;
    }
}

fn generate_call_and_return(indicator: &FuncInfo) -> TokenStream {
    let mut outputs = Vec::new();

    for output in indicator.outputs() {
        let output_ident = format_ident!("{}", rustify_input(output.name()));
        outputs.push(quote! { wrap_output(#output_ident.as_mut_ptr(), num_elements) })
    }

    // More than one output wrap in tuple
    let return_expr = if outputs.len() > 1 {
        quote! { Ok((#(#outputs),*)) }
    } else {
        quote! { Ok(#(#outputs)*) }
    };

    quote! {
        let (_begin_index, num_elements) = params.call(0, end_idx)?;
        #return_expr
    }
}

fn generate_input_type(input: &Input) -> TokenStream {
    match input.param_type() {
        InputType::Integer => quote! { AsRef<[i32]> },
        InputType::Real => quote! { AsRef<[f64]> },
        InputType::Price => {
            let flags = input.flags();
            let mut bounds = Vec::new();

            if flags.contains(InputFlags::OPEN) {
                bounds.push(quote! { Open })
            }
            if flags.contains(InputFlags::LOW) {
                bounds.push(quote! { Low })
            }
            if flags.contains(InputFlags::HIGH) {
                bounds.push(quote! { High })
            }
            if flags.contains(InputFlags::CLOSE) {
                bounds.push(quote! { Close })
            }
            if flags.contains(InputFlags::VOLUME) {
                bounds.push(quote! { Volume })
            }
            if flags.contains(InputFlags::OPEN_INTEREST) {
                bounds.push(quote! { OpenInterest })
            }

            quote! { #(#bounds)+* }
        }
    }
}

fn generate_output_type(output: &Output) -> TokenStream {
    match output.param_type() {
        OutputType::Integer => quote! { Vec<i32> },
        OutputType::Real => quote! { Vec<f64> },
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
