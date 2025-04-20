use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use validator::ValidationError;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct  WasmNewtypeValidationErrorData {
    #[wasm_bindgen(readonly)]
    pub code: String,
    #[wasm_bindgen(readonly)]
    pub message: Option<String>,
}

#[derive(Error, Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
#[error("Validation error for newtype struct")]
pub struct WasmNewtypeValidationError {
    pub errors: Vec<WasmNewtypeValidationErrorData>
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
