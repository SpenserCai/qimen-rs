//! Command-line adapter for the public Qimen and calendar libraries.

mod args;
mod render;
mod render_extensions;

use std::{io::Write, process::ExitCode};

use clap::Parser;

use args::{Command, Options};

fn run(options: Options) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = std::io::BufWriter::new(std::io::stdout().lock());
    match options.command {
        Command::Paipan(input) => {
            let chart = qimen_core::calculate_with_options(
                &input.calendar.request(),
                &input.extension_options()?,
            )?;
            if input.calendar.json {
                serde_json::to_writer_pretty(&mut output, &chart)?;
                writeln!(output)?;
            } else {
                render::chart(&mut output, &chart)?;
            }
        }
        Command::Bazi(input) => {
            let request = input.request();
            let calendar = qimen_calendar::calculate(&request)?;
            if input.json {
                serde_json::to_writer_pretty(&mut output, &calendar)?;
                writeln!(output)?;
            } else {
                render::calendar(&mut output, &request, &calendar)?;
            }
        }
    }
    output.flush()?;
    Ok(())
}

fn main() -> ExitCode {
    match run(Options::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error)
            if error
                .downcast_ref::<std::io::Error>()
                .is_some_and(|io| io.kind() == std::io::ErrorKind::BrokenPipe) =>
        {
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("错误 / error: {error}");
            ExitCode::FAILURE
        }
    }
}
