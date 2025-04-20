use proc_macro::TokenStream;
use syn::{self, parse_macro_input, Data, DeriveInput, Fields};
use quote::quote;

#[proc_macro_attribute]
pub fn wasm_newtype(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);

    let struct_data = match &input.data {
        Data::Struct(s) => s,
        _ => panic!("wasm_newtype can only be applied to structs"),
    };

    let fields = match &struct_data.fields {
        Fields::Named(fields) => fields,
        _ => panic!("wasm_newtype requires named fields"),
    };

    if fields.named.len() != 1 {
        panic!("wasm_newtype struct must have exactly one field");
    }

    let struct_name = input.ident.clone();

    let value_field = fields
        .named
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "value")
        .expect("Struct must have a field named 'value'");

    let field_type = &value_field.ty;

    let mut has_validate = false;

    for attr in &input.attrs {
        if attr.path().is_ident("derive") {
            let meta_list = attr.parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated);
            if let Ok(list) = meta_list {
                for path in list {
                    if path.is_ident("Validate") {
                        has_validate = true;
                        break;
                    }
                }
            }
        }
        if has_validate {
            break;
        }
    }

    if !has_validate {
        input.attrs.push(syn::parse_quote!( #[derive(validator::Validate)] ));
    };

    input.attrs.push(syn::parse_quote!( #[wasm_bindgen(getter_with_clone)] ));

    TokenStream::from(quote! {
        #input

        #[wasm_bindgen]
        impl #struct_name {
            #[wasm_bindgen(constructor)]
            pub fn new(value: #field_type) -> Self {
                Self { value }
            }

            #[wasm_bindgen(getter = "value")]
            pub fn get_value(&self) -> #field_type {
                self.value.clone()
            }

            #[wasm_bindgen(setter = "value")]
            pub fn set_value(&mut self, value: #field_type) {
                self.value = value
            }
        }

        #[wasm_bindgen]
        impl #struct_name {
            #[wasm_bindgen(catch)]
            pub fn safe_new(value: #field_type) -> anyhow::Result<Self, WasmNewtypeValidationError> {
                let instance = Self { value };
                let validation_result = instance.validate();

                match validation_result {
                    Ok(_) => Ok(instance),
                    Err(errors) => Err(WasmNewtypeValidationError::from(errors))
                }
            }

            #[wasm_bindgen(catch)]
            pub fn safe_set_value(&mut self, value: #field_type) -> anyhow::Result<(), WasmNewtypeValidationError> {
                self.value = value;
                let validation_result = self.validate();

                match validation_result {
                    Ok(_) => Ok(()),
                    Err(errors) => Err(WasmNewtypeValidationError::from(errors))
                }
            }
        }
    })
}
