//! Stateless MCP adapter for the calendar and Qimen libraries.
//!
//! Protocol negotiation and transports are owned by `rmcp`. This adapter only
//! describes tools, decodes their arguments, and calls the public libraries.

pub mod http;

use std::borrow::Cow;

use qimen_calendar::CalendarRequest;
use qimen_core::CalculationRequest;
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    model::{
        CacheScope, CallToolRequestParams, CallToolResponse, CallToolResult, Implementation,
        JsonObject, ListToolsResult, PaginatedRequestParams, ProtocolVersion, ServerCapabilities,
        ServerConfig, Tool, ToolAnnotations,
    },
    service::RequestContext,
};
use serde::Serialize;
use serde_json::{Value, json};

/// Preferred MCP protocol revision. Legacy revisions are negotiated by the SDK.
pub const PROTOCOL_VERSION: ProtocolVersion = ProtocolVersion::V_2026_07_28;

/// A deterministic, read-only server exposing `bazi` and `paipan` over MCP.
#[derive(Debug, Clone, Copy, Default)]
pub struct QimenServer;

fn definition(name: &str) -> Option<Tool> {
    let (name, title, description) = match name {
        "bazi" => (
            "bazi",
            "八字 / Four Pillars",
            "Calculate year, month, day and hour pillars and solar-term instants from a \
             Gregorian civil date and fixed UTC offset. Defaults: UTC+08:00, Zi-start day \
             boundary (23:00), civil time without true-solar correction.",
        ),
        "paipan" => (
            "paipan",
            "时家拆补转盘 / Hourly Qimen chart",
            "Calculate a complete hourly Chaibu rotating Qimen chart, including four pillars, \
             solar terms, dun, yuan, ju, xun, leaders, voids, horse and all nine palaces. \
             Defaults: UTC+08:00, Zi-start day boundary (23:00), civil time. Optional \
             extensions are disabled by default; select their documented conventions in \
             the extensions argument for each call.",
        ),
        _ => return None,
    };

    let tool = Tool::new(name, description, JsonObject::new())
        .with_title(title)
        .with_annotations(
            ToolAnnotations::new()
                .read_only(true)
                .destructive(false)
                .idempotent(true)
                .open_world(false),
        );
    Some(match name {
        "bazi" => tool
            .with_input_schema::<CalendarRequest>()
            .with_output_schema::<qimen_calendar::CalendarResult>(),
        _ => tool
            .with_input_schema::<CalculationRequest>()
            .with_output_schema::<qimen_core::Chart>(),
    })
}

fn result<T: Serialize, E: std::fmt::Display>(
    calculation: Result<T, E>,
) -> Result<CallToolResponse, ErrorData> {
    let response = match calculation {
        Ok(value) => CallToolResult::structured(serde_json::to_value(value).map_err(|error| {
            ErrorData::internal_error(format!("Cannot encode calculation result: {error}"), None)
        })?),
        Err(error) => CallToolResult::structured_error(json!({
            "code": "calculation_error",
            "message": error.to_string(),
        })),
    };
    Ok(response.into())
}

impl ServerHandler for QimenServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(PROTOCOL_VERSION)
            .with_server_info(
                Implementation::new("qimen-mcp", env!("CARGO_PKG_VERSION"))
                    .with_title("奇门遁甲排盘")
                    .with_website_url(env!("CARGO_PKG_REPOSITORY")),
            )
            .with_instructions(
                "Use bazi for calendrical calculations or paipan for a complete Qimen chart. \
                 Always preserve the input UTC offset and day-boundary convention when comparing \
                 charts. Optional annotations are available only on paipan through its extensions \
                 argument; they never alter the base chart. Calculations are local, deterministic \
                 and do not interpret predictions.",
            )
    }

    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Borrowed(ProtocolVersion::known_up_to(&PROTOCOL_VERSION))
    }

    async fn list_tools(
        &self,
        request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        if request.is_some_and(|params| params.cursor.is_some()) {
            return Err(ErrorData::invalid_params(
                "This server returns all tools in one page; omit cursor",
                None,
            ));
        }
        Ok(ListToolsResult::with_all_items(
            ["bazi", "paipan"]
                .into_iter()
                .filter_map(definition)
                .collect(),
        )
        .with_ttl_ms(3_600_000)
        .with_cache_scope(CacheScope::Public))
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        definition(name)
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        if !matches!(request.name.as_ref(), "bazi" | "paipan") {
            return Err(ErrorData::invalid_params(
                format!("Unknown tool: {}", request.name),
                None,
            ));
        }
        let arguments = Value::Object(request.arguments.unwrap_or_default());
        match request.name.as_ref() {
            "bazi" => {
                let input: CalendarRequest = serde_json::from_value(arguments)
                    .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
                result(qimen_calendar::calculate(&input))
            }
            _ => {
                let input: CalculationRequest = serde_json::from_value(arguments)
                    .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
                result(qimen_core::calculate_with_options(
                    &input.calendar,
                    &input.extensions,
                ))
            }
        }
    }
}
