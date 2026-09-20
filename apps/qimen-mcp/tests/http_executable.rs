//! Start the real HTTP executable with its public flags and stop it cleanly.

use std::{process::Stdio, time::Duration};

use rmcp::{
    ClientLifecycleMode, ClientServiceExt,
    model::{ClientConfig, ProtocolVersion},
    transport::StreamableHttpClientTransport,
};
use tokio::{io::AsyncBufReadExt, process::Command};

type TestResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::test]
async fn http_executable_uses_listen_and_trust_options() -> TestResult {
    tokio::time::timeout(Duration::from_secs(20), exercise()).await?
}

async fn exercise() -> TestResult {
    let mut process = Command::new(env!("CARGO_BIN_EXE_qimen-mcp"))
        .args([
            "--transport",
            "streamable-http",
            "--bind",
            "127.0.0.1:0",
            "--allow-host",
            "mcp.example.com:8443",
            "--allow-origin",
            "https://app.example.com:8443",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;
    let mut stderr = tokio::io::BufReader::new(process.stderr.take().expect("stderr")).lines();
    let started = tokio::time::timeout(Duration::from_secs(10), stderr.next_line())
        .await??
        .expect("startup message");
    let endpoint = started
        .strip_prefix("Qimen MCP listening at ")
        .expect("HTTP address");
    let transport = StreamableHttpClientTransport::from_uri(endpoint.to_owned());
    let client = ClientConfig::default()
        .serve_with_lifecycle(
            transport,
            ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            },
        )
        .await?;
    assert_eq!(client.list_tools(None).await?.tools.len(), 2);
    client.cancel().await?;
    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    for (origin, allowed) in [
        ("https://untrusted.example", false),
        ("https://app.example.com:8443", true),
    ] {
        let response = http
            .get(endpoint)
            .header("accept", "text/event-stream")
            .header("host", "mcp.example.com:8443")
            .header("origin", origin)
            .header("mcp-protocol-version", "2026-07-28")
            .send()
            .await?;
        // Modern HTTP does not provide an independent GET stream. A trusted
        // request reaches the protocol's 405; an untrusted one stops at 403.
        assert_eq!(
            response.status(),
            if allowed {
                reqwest::StatusCode::METHOD_NOT_ALLOWED
            } else {
                reqwest::StatusCode::FORBIDDEN
            }
        );
    }
    #[cfg(unix)]
    {
        let status = Command::new("kill")
            .args(["-TERM", &process.id().expect("running process").to_string()])
            .status()
            .await?;
        assert!(status.success());
        let output =
            tokio::time::timeout(Duration::from_secs(5), process.wait_with_output()).await??;
        assert!(output.status.success());
        assert!(output.stdout.is_empty());
    }
    #[cfg(not(unix))]
    process.kill().await?;
    Ok(())
}
