//! Exercise the actual HTTP endpoint through the official SDK's HTTP client.

mod common;

use std::time::Duration;

use common::{HttpServer, TestResult};
use qimen_calendar::CalendarRequest;
use qimen_mcp::PROTOCOL_VERSION;
use rmcp::{
    ClientLifecycleMode, ClientServiceExt, ServiceExt,
    model::{CallToolRequestParams, ClientConfig, ErrorCode, ProtocolVersion},
    service::ServiceError,
    transport::StreamableHttpClientTransport,
};
use serde_json::{Value, json};

fn call(name: &'static str, arguments: Value) -> CallToolRequestParams {
    CallToolRequestParams::new(name)
        .with_arguments(arguments.as_object().expect("object fixture").clone())
}

async fn exercise(version: ProtocolVersion, modern: bool) -> TestResult {
    let server = HttpServer::start(Default::default()).await;
    let transport = StreamableHttpClientTransport::from_uri(server.endpoint.clone());
    let config = ClientConfig::default().with_protocol_version(version.clone());
    let client = if modern {
        config
            .serve_with_lifecycle(
                transport,
                ClientLifecycleMode::Discover {
                    preferred_versions: vec![version.clone()],
                },
            )
            .await?
    } else {
        config.serve(transport).await?
    };
    assert_eq!(client.peer_info().expect("peer").protocol_version, version);
    let tools = client.list_tools(None).await?;
    assert_eq!(tools.tools.len(), 2);
    assert_eq!(tools.tools[0].name, "bazi");
    assert_eq!(tools.tools[1].name, "paipan");
    assert_eq!(tools.result_type.is_some(), modern);

    let request = CalendarRequest::new(2026, 9, 18, 18);
    let arguments = serde_json::to_value(&request)?;
    let bazi = client.call_tool(call("bazi", arguments.clone())).await?;
    assert_eq!(bazi.is_error, Some(false));
    assert_eq!(
        bazi.structured_content,
        Some(serde_json::to_value(qimen_calendar::calculate(&request)?)?)
    );
    let chart = client.call_tool(call("paipan", arguments.clone())).await?;
    assert_eq!(chart.is_error, Some(false));
    assert_eq!(
        chart.structured_content,
        Some(serde_json::to_value(qimen_core::calculate(&request)?)?)
    );
    let extensions = qimen_core::ExtensionOptions::all();
    let mut extended = arguments;
    extended["extensions"] = serde_json::to_value(&extensions)?;
    let chart = client.call_tool(call("paipan", extended)).await?;
    assert_eq!(
        chart.structured_content,
        Some(serde_json::to_value(qimen_core::calculate_with_options(
            &request,
            &extensions,
        )?)?)
    );
    for (name, args) in [
        ("missing", json!({})),
        ("bazi", json!({})),
        (
            "paipan",
            json!({"year": 2026, "month": 9, "day": 18, "hour": 18,
                   "extensions": {"day_horse": "unknown"}}),
        ),
    ] {
        assert!(matches!(
            client.call_tool(call(name, args)).await,
            Err(ServiceError::McpError(error)) if error.code == ErrorCode::INVALID_PARAMS
        ));
    }
    let invalid = client
        .call_tool(call(
            "paipan",
            json!({"year": 2026, "month": 2, "day": 30, "hour": 18}),
        ))
        .await?;
    assert_eq!(invalid.is_error, Some(true));
    assert_eq!(
        invalid.structured_content.expect("calculation error")["code"],
        "calculation_error"
    );
    client.cancel().await?;
    server.stop().await
}

#[tokio::test]
async fn modern_http_discovery_and_tools() -> TestResult {
    tokio::time::timeout(Duration::from_secs(20), exercise(PROTOCOL_VERSION, true)).await?
}

#[tokio::test]
async fn legacy_http_initialization_and_tools() -> TestResult {
    for version in [
        ProtocolVersion::V_2025_03_26,
        ProtocolVersion::V_2025_06_18,
        ProtocolVersion::V_2025_11_25,
    ] {
        tokio::time::timeout(Duration::from_secs(20), exercise(version, false)).await??;
    }
    Ok(())
}
