//! Extension arguments must preserve the core contract and remain opt-in.

use std::process::{Command, Output};

use qimen_core::{CalendarRequest, ExtensionOptions};
use serde_json::Value;

fn run(command: &str, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_qimen"))
        .args([
            command, "--year", "2026", "--month", "9", "--day", "18", "--hour", "18", "--minute",
            "15",
        ])
        .args(extra)
        .output()
        .expect("run qimen binary")
}

fn json(extra: &[&str]) -> Value {
    let output = run("paipan", extra);
    assert!(output.status.success(), "{:?}", output);
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).expect("only JSON on stdout")
}

#[test]
fn defaults_omit_extensions_and_all_matches_the_core_result() {
    assert!(json(&["--json"]).get("extensions").is_none());
    let actual = json(&["--extensions", "all", "--json"]);
    let mut request = CalendarRequest::new(2026, 9, 18, 18);
    request.minute = 15;
    let expected = qimen_core::calculate_with_options(&request, &ExtensionOptions::all())
        .expect("valid chart");
    assert_eq!(actual, serde_json::to_value(expected).expect("chart JSON"));
}

#[test]
fn individual_and_combined_selection_only_enables_requested_annotations() {
    for (argument, field) in [
        ("hidden-stems", "hidden_stems"),
        ("strength", "strength"),
        ("growth-stages", "growth_stages"),
        ("punishments", "punishments"),
        ("tombs", "tombs"),
        ("day-horse", "day_horse"),
        ("door-pressure", "door_pressure"),
    ] {
        let actual = json(&["--extensions", argument, "--json"]);
        let extensions = actual["extensions"].as_object().expect("extensions object");
        assert_eq!(extensions.len(), 1, "{argument}");
        assert!(extensions.contains_key(field), "{argument}");
    }
    let actual = json(&["--extensions", "day-horse,hidden-stems", "--json"]);
    let extensions = actual["extensions"].as_object().expect("extensions object");
    assert_eq!(extensions.len(), 2);
    assert!(extensions.contains_key("day_horse"));
    assert!(extensions.contains_key("hidden_stems"));
}

#[test]
fn invalid_or_inapplicable_extension_arguments_are_rejected() {
    for (command, extra) in [
        ("paipan", vec!["--extensions", "unknown"]),
        ("paipan", vec!["--extensions", ""]),
        ("bazi", vec!["--extensions", "all"]),
        ("paipan", vec!["--tomb-rule", "traditional-three-wonders"]),
        (
            "paipan",
            vec![
                "--extensions",
                "day-horse",
                "--tomb-rule",
                "traditional-three-wonders",
            ],
        ),
        (
            "paipan",
            vec!["--extensions", "tombs", "--tomb-rule", "unknown"],
        ),
    ] {
        let output = run(command, &extra);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
}

#[test]
fn tomb_convention_is_explicit_and_preserves_non_applicability() {
    for selection in ["tombs", "all"] {
        let actual = json(&[
            "--extensions",
            selection,
            "--tomb-rule",
            "traditional-three-wonders",
            "--json",
        ]);
        let tombs = &actual["extensions"]["tombs"];
        assert_eq!(tombs["rule"], "traditional_three_wonders");
        for stem in tombs["stems"].as_array().expect("tomb occurrences") {
            let stem_name = stem["placement"]["stem"].as_str().expect("stem enum");
            if !matches!(stem_name, "yi" | "bing" | "ding") {
                assert!(stem["is_in_tomb"].is_null());
                assert!(stem["tomb_branch"].is_null());
            }
        }
    }
    let output = run(
        "paipan",
        &[
            "--extensions",
            "tombs",
            "--tomb-rule",
            "traditional-three-wonders",
        ],
    );
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).expect("UTF-8 chart");
    assert!(text.contains("传统三奇入墓"));
    assert!(text.contains("所选规则不适用"));
}

#[test]
fn help_describes_the_conventions_and_text_renders_selected_annotations() {
    let output = Command::new(env!("CARGO_BIN_EXE_qimen"))
        .args(["paipan", "--help"])
        .output()
        .expect("run help");
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).expect("UTF-8 help");
    for expected in ["--extensions", "hidden-stems", "growth-stages", "乙墓戌"] {
        assert!(help.contains(expected), "missing {expected}: {help}");
    }

    let plain = run("paipan", &[]);
    let plain = String::from_utf8(plain.stdout).expect("UTF-8 chart");
    assert!(!plain.contains("扩展明细"));
    let extended = run("paipan", &["--extensions", "all"]);
    assert!(extended.status.success(), "{:?}", extended);
    let text = String::from_utf8(extended.stdout).expect("UTF-8 extended chart");
    for expected in [
        "扩展明细",
        "暗干",
        "旺衰",
        "十二长生",
        "击刑",
        "入墓",
        "日马",
        "门迫",
        "寄",
    ] {
        assert!(text.contains(expected), "missing {expected}: {text}");
    }
}
