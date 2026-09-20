//! Executable argument checks for transport selection and HTTP configuration.

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_qimen-mcp"))
        .args(args)
        .output()
        .expect("run MCP executable")
}

#[test]
fn help_describes_both_transports_and_http_security() {
    let output = run(&["--help"]);
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).expect("UTF-8 help");
    for expected in [
        "stdio",
        "streamable-http",
        "127.0.0.1:8080",
        "/mcp",
        "--allow-host",
        "--allow-origin",
        "authentication",
    ] {
        assert!(help.contains(expected), "missing {expected}: {help}");
    }
}

#[test]
fn http_options_require_the_http_transport() {
    for args in [
        vec!["--bind", "127.0.0.1:8080"],
        vec!["--allow-host", "mcp.example.com"],
        vec![
            "--transport",
            "stdio",
            "--allow-origin",
            "https://app.example.com",
        ],
    ] {
        let output = run(&args);
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("require --transport"));
    }
}

#[test]
fn invalid_http_addresses_and_allowlists_are_rejected_before_listening() {
    for (flag, value) in [
        ("--bind", "localhost:8080"),
        ("--allow-host", "*"),
        ("--allow-host", "user@example.com"),
        ("--allow-host", "https://example.com"),
        ("--allow-host", "example.com:99999"),
        ("--allow-origin", "null"),
        ("--allow-origin", "https://*.example.com"),
        ("--allow-origin", "https://example.com/mcp"),
        ("--allow-origin", "https://example.com?x=1"),
        ("--allow-origin", "https://user@example.com"),
    ] {
        let output = run(&["--transport", "streamable-http", flag, value]);
        assert_eq!(output.status.code(), Some(2), "{flag} {value}: {output:?}");
        assert!(output.stdout.is_empty());
    }
}
