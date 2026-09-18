//! Python ABI adapter. All validation and calendrical work belong to `qimen-core`.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Calculate a chart using the canonical JSON request and response schema.
///
/// # Errors
///
/// Returns `ValueError` when the request is invalid or cannot be calculated.
#[pyfunction]
fn calculate_json(py: Python<'_>, request_json: String) -> PyResult<String> {
    py.detach(move || qimen_core::calculate_json(&request_json))
        .map_err(|error| PyValueError::new_err(error.to_string()))
}

/// Native qimen-rs module; Python conveniences live in the surrounding package.
#[pymodule]
mod _native {
    #[pymodule_export]
    use super::calculate_json;
}
