//! Public configuration and strict JSON compatibility at the adapter boundary.

use qimen_core::{
    CalculationRequest, Calculator, Chart, ChartRequest, ExtensionOptions, calculate,
    calculate_json,
};
use serde_json::{Value, json};

#[test]
fn omitted_or_empty_extensions_preserve_base_results() {
    let expected = calculate(&ChartRequest::new(2026, 9, 18, 18)).unwrap();
    for input in [
        r#"{"year":2026,"month":9,"day":18,"hour":18}"#,
        r#"{"year":2026,"month":9,"day":18,"hour":18,"extensions":{}}"#,
    ] {
        let output = calculate_json(input).unwrap();
        assert_eq!(serde_json::from_str::<Chart>(&output).unwrap(), expected);
        assert!(
            serde_json::from_str::<Value>(&output)
                .unwrap()
                .get("extensions")
                .is_none()
        );
    }
    assert_eq!(
        Calculator::default().calculate(&expected.input).unwrap(),
        expected
    );
}

#[test]
fn initialized_calculator_annotations_do_not_change_the_base_chart() {
    let request = ChartRequest::new(2026, 9, 18, 18);
    let expected = calculate(&request).unwrap();
    let calculator = Calculator::new(ExtensionOptions::all());
    let mut chart = calculator.calculate(&request).unwrap();
    assert!(chart.extensions.take().is_some());
    assert_eq!(chart, expected);
    assert_eq!(calculator.extensions(), &ExtensionOptions::all());
}

#[test]
fn configured_request_keeps_flat_civil_fields_and_strict_rules() {
    let input = json!({
        "year": 2026, "month": 9, "day": 18, "hour": 18,
        "extensions": {"day_horse": "day_branch_three_harmony"}
    });
    let request: CalculationRequest = serde_json::from_value(input).unwrap();
    assert_eq!(request.calendar, ChartRequest::new(2026, 9, 18, 18));
    let encoded = serde_json::to_value(&request).unwrap();
    assert_eq!(encoded["year"], 2026);
    assert!(encoded.get("calendar").is_none());
    assert_eq!(
        encoded["extensions"]["day_horse"],
        "day_branch_three_harmony"
    );
    assert_eq!(
        serde_json::from_value::<CalculationRequest>(encoded).unwrap(),
        request
    );
}

#[test]
fn unknown_duplicate_and_malformed_options_are_rejected() {
    for fields in [
        r#""timezone":480"#,
        r#""extensions":{"day_hrose":"day_branch_three_harmony"}"#,
        r#""extensions":{"day_horse":"unknown"}"#,
        r#""extensions":{"day_horse":true}"#,
        r#""extensions":{"day_horse":{"day_branch_three_harmony":null}}"#,
        r#""day_boundary":{"zi_start":null}"#,
        r#""extensions":null"#,
        r#""extensions":[]"#,
        r#""year":2025"#,
        r#""extensions":{},"extensions":{}"#,
        r#""extensions":{"hidden_stems":null,"hidden_stems":"duty_door_hour_stem_with_center_fallback"}"#,
    ] {
        let input = format!(r#"{{"year":2026,"month":9,"day":18,"hour":18,{fields}}}"#);
        assert!(
            calculate_json(&input).is_err(),
            "accepted invalid request: {input}"
        );
    }
}

#[test]
fn version_one_charts_without_extensions_remain_readable() {
    let mut old =
        serde_json::to_value(calculate(&ChartRequest::new(2026, 9, 18, 18)).unwrap()).unwrap();
    old["schema_version"] = json!("1.0");
    assert!(old.get("extensions").is_none());
    let decoded: Chart = serde_json::from_value(old).unwrap();
    assert!(decoded.extensions.is_none());
    assert_eq!(decoded.schema_version, "1.0");
}
