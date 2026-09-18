//! Node-API adapter for the canonical `qimen-core` JSON boundary.

use napi::{Error, Result, Status};
use napi_derive::napi;

/// Calculate a chart using the canonical JSON request and response schema.
///
/// # Errors
///
/// Throws a JavaScript error with code `InvalidArg` for invalid requests.
#[napi(js_name = "calculateJson")]
pub fn calculate_json(request_json: String) -> Result<String> {
    qimen_core::calculate_json(&request_json)
        .map_err(|error| Error::new(Status::InvalidArg, error.to_string()))
}
