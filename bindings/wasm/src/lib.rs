//! Browser/WebAssembly adapters for the shared Rust chart engine.

use serde::Serialize;
use wasm_bindgen::prelude::*;

/// Calculate a chart from the same request object as all other bindings.
///
/// # Errors
///
/// Throws a JavaScript `Error` for invalid input or serialization failures.
#[wasm_bindgen(skip_typescript)]
pub fn calculate(request: JsValue) -> Result<JsValue, JsError> {
    // Direct struct deserialization only visits declared fields in
    // serde-wasm-bindgen, bypassing serde's deny_unknown_fields validation.
    // Preserve every object key before applying the canonical request schema.
    let request: serde_json::Value = serde_wasm_bindgen::from_value(request)?;
    let request: qimen_core::CalculationRequest = serde_json::from_value(request)?;
    let chart = request
        .calculate()
        .map_err(|error| JsError::new(&error.to_string()))?;
    Ok(chart.serialize(&serde_wasm_bindgen::Serializer::json_compatible())?)
}

/// Calculate a chart using the canonical JSON request and response schema.
///
/// # Errors
///
/// Throws a JavaScript `Error` for malformed JSON or an invalid request.
#[wasm_bindgen(js_name = calculateJson)]
pub fn calculate_json(request_json: &str) -> Result<String, JsError> {
    qimen_core::calculate_json(request_json).map_err(|error| JsError::new(&error.to_string()))
}

#[wasm_bindgen(typescript_custom_section)]
const SHARED_TYPES: &str = include_str!("../../node/schema.d.ts");

#[wasm_bindgen(typescript_custom_section)]
const CALCULATE_TYPE: &str = "export function calculate(request: ChartRequest): Chart;";
