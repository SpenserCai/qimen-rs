//! End-to-end checks for CLI arguments, JSON fidelity and readable palace layout.

use std::process::{Command, Output};

use qimen_calendar::{CalendarRequest, DayBoundary};
use serde_json::Value;
use unicode_width::UnicodeWidthStr;

fn run(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_qimen"))
        .args(arguments)
        .output()
        .expect("run qimen binary")
}

fn date_arguments(command: &str) -> Vec<&str> {
    vec![
        command, "--year", "2024", "--month", "2", "--day", "10", "--hour", "12",
    ]
}

#[test]
fn help_exposes_commands_and_explicit_time_conventions() {
    let output = run(&["--help"]);
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).expect("UTF-8 help");
    for expected in ["paipan", "bazi", "UTC+08:00", "23:00", "--json"] {
        assert!(text.contains(expected), "missing {expected}: {text}");
    }
    let output = run(&["paipan", "--help"]);
    let text = String::from_utf8(output.stdout).expect("UTF-8 subcommand help");
    assert!(text.contains("--utc-offset-minutes"));
    assert!(text.contains("--day-boundary"));
}

#[test]
fn json_outputs_match_the_public_library_results() {
    let request = CalendarRequest::new(2024, 2, 10, 12);
    for (command, expected) in [
        (
            "paipan",
            serde_json::to_value(qimen_core::calculate(&request).expect("valid chart"))
                .expect("chart JSON"),
        ),
        (
            "bazi",
            serde_json::to_value(qimen_calendar::calculate(&request).expect("valid calendar"))
                .expect("calendar JSON"),
        ),
    ] {
        let mut arguments = date_arguments(command);
        arguments.push("--json");
        let output = run(&arguments);
        assert!(output.status.success(), "{:?}", output);
        assert!(output.stderr.is_empty());
        let value: Value = serde_json::from_slice(&output.stdout).expect("only JSON on stdout");
        assert_eq!(value, expected);
    }
}

#[test]
fn explicit_offset_seconds_and_rollover_reach_the_library() {
    let output = run(&[
        "paipan",
        "--year",
        "2024",
        "--month",
        "2",
        "--day",
        "10",
        "--hour",
        "23",
        "--minute",
        "59",
        "--second",
        "58",
        "--utc-offset-minutes",
        "-300",
        "--day-boundary",
        "midnight",
        "--json",
    ]);
    assert!(output.status.success(), "{:?}", output);
    let mut request = CalendarRequest::new(2024, 2, 10, 23);
    request.minute = 59;
    request.second = 58;
    request.utc_offset_minutes = -300;
    request.day_boundary = DayBoundary::Midnight;
    let expected = qimen_core::calculate(&request).expect("valid chart");
    let actual: Value = serde_json::from_slice(&output.stdout).expect("chart JSON");
    assert_eq!(
        actual,
        serde_json::to_value(expected).expect("expected JSON")
    );
}

#[test]
fn invalid_inputs_fail_without_partial_stdout() {
    for arguments in [
        vec![
            "paipan", "--year", "2024", "--month", "2", "--day", "30", "--hour", "12",
        ],
        vec![
            "bazi", "--year", "2024", "--month", "2", "--day", "10", "--hour", "24",
        ],
        vec![
            "paipan",
            "--year",
            "2024",
            "--month",
            "2",
            "--day",
            "10",
            "--hour",
            "12",
            "--day-boundary",
            "unknown",
        ],
    ] {
        let output = run(&arguments);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
}

#[test]
fn human_chart_renders_all_nine_palaces_with_equal_display_width() {
    let output = run(&date_arguments("paipan"));
    assert!(output.status.success(), "{:?}", output);
    let text = String::from_utf8(output.stdout).expect("UTF-8 chart");
    for expected in [
        "时家拆补转盘",
        "八字",
        "值符",
        "值使",
        "旬首",
        "空亡",
        "驿马",
    ] {
        assert!(text.contains(expected), "missing {expected}");
    }
    for palace in 1..=9 {
        assert!(text.contains(&format!("{palace}宫")));
    }
    let widths: Vec<_> = text
        .lines()
        .filter(|line| line.starts_with(['│', '┌', '├', '└']))
        .map(UnicodeWidthStr::width)
        .collect();
    assert_eq!(widths.len(), 25);
    assert!(widths.iter().all(|width| *width == widths[0]));
    let rows: Vec<_> = text
        .lines()
        .filter(|line| line.starts_with('│') && line.contains("宫 "))
        .collect();
    assert!(rows[0].find("4宫") < rows[0].find("9宫"));
    assert!(rows[0].find("9宫") < rows[0].find("2宫"));
}
