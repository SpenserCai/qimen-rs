//! HTTP envelope, session lifecycle, Origin and Host validation.

mod common;

use std::time::Duration;

use common::{HttpServer, TestResult};
use reqwest::{Client, RequestBuilder, StatusCode};
use rmcp::transport::streamable_http_server::StreamableHttpServerConfig;
use serde_json::{Value, json};

fn client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("HTTP client")
}

fn post(client: &Client, url: &str, body: &Value) -> RequestBuilder {
    client
        .post(url)
        .header("accept", "application/json, text/event-stream")
        .json(body)
}

fn initialize() -> Value {
    json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
        "protocolVersion": "2025-11-25", "capabilities": {},
        "clientInfo": {"name": "http-test", "version": "1.0"}
    }})
}

#[tokio::test]
async fn untrusted_origins_and_hosts_are_rejected_on_all_mcp_methods() -> TestResult {
    let server = HttpServer::start(Default::default()).await;
    let client = client();
    for method in [
        reqwest::Method::POST,
        reqwest::Method::GET,
        reqwest::Method::DELETE,
    ] {
        for (header, value) in [
            ("origin", "https://untrusted.example"),
            ("origin", "null"),
            ("origin", "malformed"),
            ("host", "rebound.example"),
        ] {
            let response = client
                .request(method.clone(), &server.endpoint)
                .header(header, value)
                .json(&initialize())
                .send()
                .await?;
            assert_eq!(
                response.status(),
                StatusCode::FORBIDDEN,
                "{method} {header}"
            );
        }
    }
    assert_eq!(
        client
            .get(format!("{}/extra", server.endpoint))
            .send()
            .await?
            .status(),
        StatusCode::NOT_FOUND
    );
    server.stop().await
}

#[tokio::test]
async fn explicitly_trusted_proxy_host_and_origin_reach_the_protocol() -> TestResult {
    let config = StreamableHttpServerConfig::default()
        .with_allowed_hosts(["mcp.example.com:8443"])
        .with_allowed_origins(["https://app.example.com:8443"]);
    let server = HttpServer::start(config).await;
    let client = client();
    for origin in ["https://app.example.com:443", "http://app.example.com:8443"] {
        assert_eq!(
            post(&client, &server.endpoint, &initialize())
                .header("host", "mcp.example.com:8443")
                .header("origin", origin)
                .send()
                .await?
                .status(),
            StatusCode::FORBIDDEN
        );
    }
    let response = post(&client, &server.endpoint, &initialize())
        .header("host", "mcp.example.com:8443")
        .header("origin", "https://app.example.com:8443")
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().contains_key("mcp-session-id"));
    drop(response);
    server.stop().await
}

#[tokio::test]
async fn legacy_sessions_support_sse_delete_and_expired_session_errors() -> TestResult {
    let server = HttpServer::start(Default::default()).await;
    let client = client();
    let response = post(&client, &server.endpoint, &initialize())
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    let session = response.headers()["mcp-session-id"].to_str()?.to_owned();
    assert!(response.text().await?.contains("2025-11-25"));
    let initialized = json!({"jsonrpc": "2.0", "method": "notifications/initialized"});
    let response = post(&client, &server.endpoint, &initialized)
        .header("mcp-session-id", &session)
        .header("mcp-protocol-version", "2025-11-25")
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    assert!(response.bytes().await?.is_empty());

    let stream = client
        .get(&server.endpoint)
        .header("accept", "text/event-stream")
        .header("mcp-session-id", &session)
        .header("mcp-protocol-version", "2025-11-25")
        .send()
        .await?;
    assert_eq!(stream.status(), StatusCode::OK);
    assert_eq!(stream.headers()["content-type"], "text/event-stream");
    drop(stream);

    let response = client
        .delete(&server.endpoint)
        .header("mcp-session-id", &session)
        .header("mcp-protocol-version", "2025-11-25")
        .send()
        .await?;
    assert!(response.status().is_success());
    let tools = json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"});
    assert_eq!(
        post(&client, &server.endpoint, &tools)
            .header("mcp-session-id", &session)
            .header("mcp-protocol-version", "2025-11-25")
            .send()
            .await?
            .status(),
        StatusCode::NOT_FOUND
    );
    server.stop().await
}

#[tokio::test]
async fn modern_discovery_is_stateless_and_rejects_mismatched_protocol_metadata() -> TestResult {
    let server =
        HttpServer::start(StreamableHttpServerConfig::default().with_json_response(true)).await;
    let client = client();
    let body = json!({"jsonrpc": "2.0", "id": 1, "method": "server/discover", "params": {
        "_meta": {
            "io.modelcontextprotocol/protocolVersion": "2026-07-28",
            "io.modelcontextprotocol/clientInfo": {"name": "http-test", "version": "1.0"},
            "io.modelcontextprotocol/clientCapabilities": {}
        }
    }});
    let response = post(&client, &server.endpoint, &body)
        .header("mcp-protocol-version", "2026-07-28")
        .header("mcp-method", "server/discover")
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(!response.headers().contains_key("mcp-session-id"));
    let data: Value = response.json().await?;
    assert!(
        data["result"]["supportedVersions"]
            .as_array()
            .expect("supported versions")
            .contains(&json!("2026-07-28"))
    );
    assert_eq!(
        data["result"]["_meta"]["io.modelcontextprotocol/serverInfo"]["name"],
        "qimen-mcp"
    );
    let mismatched = post(&client, &server.endpoint, &body)
        .header("mcp-protocol-version", "2025-11-25")
        .header("mcp-method", "server/discover")
        .send()
        .await?;
    assert_eq!(mismatched.status(), StatusCode::BAD_REQUEST);
    server.stop().await
}
