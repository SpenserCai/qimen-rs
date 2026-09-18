//! Wire-level integration coverage through the official SDK's async transport.

use std::time::Duration;

use qimen_calendar::CalendarRequest;
use qimen_mcp::{PROTOCOL_VERSION, QimenServer};
use rmcp::{
    ClientHandler, ClientLifecycleMode, ClientServiceExt, ServerHandler, ServiceExt,
    model::{
        CallToolRequestParams, ClientCapabilities, ClientConfig, ErrorCode, Implementation,
        ProtocolVersion,
    },
    service::ServiceError,
};
use serde_json::{Value, json};

type TestResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
struct TestClient(ProtocolVersion);

impl ClientHandler for TestClient {
    fn get_info(&self) -> ClientConfig {
        ClientConfig::new(
            ClientCapabilities::default(),
            Implementation::new("qimen-integration-test", "0.1.0"),
        )
        .with_protocol_version(self.0.clone())
    }
}

fn call(name: &'static str, arguments: Value) -> CallToolRequestParams {
    CallToolRequestParams::new(name)
        .with_arguments(arguments.as_object().expect("object fixture").clone())
}

async fn exercise(version: ProtocolVersion, modern: bool) -> TestResult {
    let (server_transport, client_transport) = tokio::io::duplex(65_536);
    let server_task = tokio::spawn(async move { QimenServer.serve(server_transport).await });
    let client = if modern {
        TestClient(version.clone())
            .serve_with_lifecycle(
                client_transport,
                ClientLifecycleMode::Discover {
                    preferred_versions: vec![version.clone()],
                },
            )
            .await?
    } else {
        TestClient(version.clone()).serve(client_transport).await?
    };

    assert_eq!(
        client
            .peer_info()
            .expect("negotiated peer info")
            .protocol_version,
        version
    );
    let tools = client.list_tools(None).await?;
    assert_eq!(tools.tools.len(), 2);
    assert_eq!(tools.tools[0].name, "bazi");
    assert_eq!(tools.tools[1].name, "paipan");
    if modern {
        assert!(tools.result_type.is_some());
        assert_eq!(tools.ttl_ms, Some(3_600_000));
    } else {
        assert!(tools.result_type.is_none());
    }

    let arguments = json!({"year": 2024, "month": 2, "day": 10, "hour": 12});
    let request: CalendarRequest = serde_json::from_value(arguments.clone())?;
    let bazi = client.call_tool(call("bazi", arguments.clone())).await?;
    assert_eq!(bazi.is_error, Some(false));
    assert_eq!(
        bazi.structured_content,
        Some(serde_json::to_value(qimen_calendar::calculate(&request)?)?)
    );
    let chart = client.call_tool(call("paipan", arguments)).await?;
    assert_eq!(chart.is_error, Some(false));
    assert_eq!(
        chart.structured_content,
        Some(serde_json::to_value(qimen_core::calculate(&request)?)?)
    );
    assert_eq!(chart.result_type.is_some(), modern);
    assert!(
        chart
            .content
            .first()
            .and_then(|item| item.as_text())
            .is_some()
    );

    let invalid_date = client
        .call_tool(call(
            "paipan",
            json!({"year": 2024, "month": 2, "day": 30, "hour": 12}),
        ))
        .await?;
    assert_eq!(invalid_date.is_error, Some(true));
    assert_eq!(
        invalid_date.structured_content.expect("structured error")["code"],
        "calculation_error"
    );

    let malformed = client.call_tool(call("bazi", json!({}))).await;
    assert!(matches!(
        malformed,
        Err(ServiceError::McpError(error)) if error.code == ErrorCode::INVALID_PARAMS
    ));
    let unknown = client.call_tool(call("missing", json!({}))).await;
    assert!(matches!(
        unknown,
        Err(ServiceError::McpError(error)) if error.code == ErrorCode::INVALID_PARAMS
    ));

    client.cancel().await?;
    let server = server_task.await??;
    server.cancel().await?;
    Ok(())
}

#[tokio::test]
async fn modern_discovery_and_tool_execution() -> TestResult {
    tokio::time::timeout(Duration::from_secs(20), exercise(PROTOCOL_VERSION, true)).await?
}

#[tokio::test]
async fn all_legacy_handshakes_remain_compatible() -> TestResult {
    for version in [
        ProtocolVersion::V_2024_11_05,
        ProtocolVersion::V_2025_03_26,
        ProtocolVersion::V_2025_06_18,
        ProtocolVersion::V_2025_11_25,
    ] {
        tokio::time::timeout(Duration::from_secs(20), exercise(version, false)).await??;
    }
    Ok(())
}

#[test]
fn tool_schemas_and_capabilities_describe_read_only_local_calculations() {
    let server = QimenServer;
    assert_eq!(server.get_info().protocol_version, PROTOCOL_VERSION);
    for name in ["bazi", "paipan"] {
        let tool = server.get_tool(name).expect("registered tool");
        let schema = tool.schema_as_json_value();
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["additionalProperties"], false);
        assert!(schema["properties"].get("utc_offset_minutes").is_some());
        assert!(schema["properties"].get("day_boundary").is_some());
        let annotations = tool.annotations.expect("explicit tool behavior");
        assert_eq!(annotations.read_only_hint, Some(true));
        assert_eq!(annotations.open_world_hint, Some(false));
    }
    assert!(server.get_tool("missing").is_none());
}
