//! Standard-input/output entry point for the Qimen MCP server.

use clap::Parser;
use qimen_mcp::QimenServer;
use rmcp::{ServiceExt, transport::stdio};

#[derive(Parser)]
#[command(
    version,
    about = "奇门遁甲 MCP 服务 / Qimen Dunjia MCP server",
    long_about = "Run the read-only bazi and paipan tools over standard input/output.\n\
                  MCP 2026-07-28 discovery and legacy initialization are supported.\n\
                  Standard output is reserved for MCP JSON-RPC messages."
)]
struct Options {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _options = Options::parse();
    QimenServer.serve(stdio()).await?.waiting().await?;
    Ok(())
}
