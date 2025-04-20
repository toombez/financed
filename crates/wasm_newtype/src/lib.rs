use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::{Validate, ValidationErrors};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use validator::ValidationError;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct WasmNewtypeValidationErrorData {
    #[wasm_bindgen(readonly)]
    pub code: String,
    #[wasm_bindgen(readonly)]
    pub message: Option<String>,
}

#[wasm_bindgen]
impl WasmNewtypeValidationErrorData {
    #[wasm_bindgen(getter)]
    pub fn is_has_message(&self) -> bool {
        match &self.message {
            Some(_) => true,
            None => false,
        }
    }
}

#[derive(Error, Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
#[error("Validation error for newtype struct")]
pub struct WasmNewtypeValidationError {
    pub errors: Vec<WasmNewtypeValidationErrorData>
}

impl From<ValidationErrors> for WasmNewtypeValidationError {
    fn from(value: ValidationErrors) -> Self {
        value
            .field_errors()
            .get("value")
            .map(|errors|
                WasmNewtypeValidationError { errors: errors
                    .iter()
                    .map(WasmNewtypeValidationErrorData::from)
                    .collect()
            })
            .unwrap()
    }
}

impl From<&ValidationErrors> for WasmNewtypeValidationError {
    fn from(value: &ValidationErrors) -> Self {
        Self::from(value.clone())
    }
}

impl From<ValidationError> for WasmNewtypeValidationErrorData {
    fn from(value: ValidationError) -> Self {
        let code = value
            .code
            .to_string();
        let message = value
            .message
            .map(|message| message.to_string());

        Self {
            code,
            message,
        }
    }
}

impl From<&ValidationError> for WasmNewtypeValidationErrorData {
    fn from(value: &ValidationError) -> Self {
        Self::from(value.clone())
    }
}

pub trait WasmNewtype<T: Into<JsValue>>: Validate {
    fn new(value: T) -> Self;
    fn get_value(&self) -> T;
    fn set_value(&mut self, value: T) -> ();

    fn safe_new(value: T) -> Result<Self, WasmNewtypeValidationError> where Self: Sized;
    fn safe_set_value(&mut self, value: T) -> Result<Self, WasmNewtypeValidationError> where Self: Sized;
}
