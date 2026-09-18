//! Print the canonical JSON schemas: `cargo run -p qimen-core --example schema
//! --features schema -- request` (or `chart`).

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kind = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "chart".to_owned());
    let schema = match kind.as_str() {
        "request" => qimen_core::request_schema(),
        "chart" => qimen_core::chart_schema(),
        _ => return Err("expected one argument: request or chart".into()),
    };
    println!("{}", serde_json::to_string_pretty(&schema)?);
    Ok(())
}
