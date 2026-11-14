use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use relay_rover::benchmark::{BenchmarkRunner, print_benchmark_report};
use relay_rover::engine::EmailEngine;
use relay_rover::models::EmailRecord;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Check for CLI argument dispatch (e.g. for Node.js / React Native / script integration)
    if args.len() > 1 {
        let engine = EmailEngine::new().context("Failed to initialize Relay Rover Engine")?;

        match args[1].as_str() {
            "--classify" => {
                let subject = args.get(2).map(|s| s.as_str()).unwrap_or("");
                let body = args.get(3).map(|s| s.as_str()).unwrap_or("");
                let processed = engine.process_raw(subject, body);
                println!("{}", processed.primary_category.as_str());
                return Ok(());
            }
            "--preview" => {
                let subject = args.get(2).map(|s| s.as_str()).unwrap_or("");
                let body = args.get(3).map(|s| s.as_str()).unwrap_or("");
                let processed = engine.process_raw(subject, body);
                println!("{}", processed.preview);
                return Ok(());
            }
            "--process" => {
                let subject = args.get(2).map(|s| s.as_str()).unwrap_or("");
                let body = args.get(3).map(|s| s.as_str()).unwrap_or("");
                let processed = engine.process_raw(subject, body);
                println!("{}", serde_json::to_string_pretty(&processed)?);
                return Ok(());
            }
            "--process-json" => {
                let json_input = args.get(2).map(|s| s.as_str()).unwrap_or("{}");
                let res = relay_rover::wasm::process_email_json_internal(json_input)
                    .map_err(|e| anyhow::anyhow!(e))?;
                println!("{}", res);
                return Ok(());
            }
            "--help" | "-h" => {
                println!("Relay Rover CLI Engine");
                println!("Usage:");
                println!(
                    "  relay-rover                              Run benchmark (single-threaded default)"
                );
                println!(
                    "  relay-rover --threads <N>                Run benchmark with N worker threads"
                );
                println!(
                    "  relay-rover --multi-thread [N]           Run benchmark with multi-threading enabled"
                );
                println!(
                    "  relay-rover --single-thread              Run benchmark in single-threaded mode"
                );
                println!("  relay-rover --classify <sub> <bod>       Fast category classification");
                println!(
                    "  relay-rover --preview <sub> <bod>        Generate notification preview"
                );
                println!("  relay-rover --process <sub> <bod>        Full typed extraction JSON");
                println!("  relay-rover --process-json <json>        Process email JSON object");
                return Ok(());
            }
            _ => {}
        }
    }

    // Parse threading flags (Default: Single-threaded)
    let mut multithread = false;
    let mut num_threads: Option<usize> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--threads" => {
                if let Some(val) = args.get(i + 1) {
                    if let Ok(n) = val.parse::<usize>() {
                        num_threads = Some(n);
                        multithread = true;
                    }
                    i += 1;
                }
            }
            "--multi-thread" => {
                multithread = true;
                if let Some(val) = args.get(i + 1)
                    && let Ok(n) = val.parse::<usize>()
                {
                    num_threads = Some(n);
                    i += 1;
                }
            }
            "--single-thread" => {
                multithread = false;
                num_threads = None;
            }
            _ => {}
        }
        i += 1;
    }

    let dataset_path = Path::new("data/emails.json");
    if !dataset_path.exists() {
        eprintln!(
            "Error: Dataset file not found at {}",
            dataset_path.display()
        );
        std::process::exit(1);
    }

    let file = File::open(dataset_path)
        .with_context(|| format!("Failed to open dataset at {}", dataset_path.display()))?;
    let reader = BufReader::new(file);
    let emails: Vec<EmailRecord> =
        serde_json::from_reader(reader).context("Failed to parse emails.json")?;

    println!(
        "Loaded {} synthetic emails from data/emails.json",
        emails.len()
    );

    // Initialize Rover Engine with user-chosen or default threading
    let engine = relay_rover::RoverBuilder::new()
        .with_threading(multithread, num_threads)
        .build()
        .context("Failed to initialize Relay Rover Engine")?;

    println!(
        "Execution Mode:            {} ({} worker thread{})",
        if engine.is_multi_threaded() {
            "Multi-threaded"
        } else {
            "Single-threaded (default)"
        },
        engine.num_threads(),
        if engine.num_threads() == 1 { "" } else { "s" }
    );

    // Demonstrate sample outputs from the engine
    println!("\n--- SAMPLE PROCESSED PREVIEWS ---");
    let sample_indices = [0, 15, 25, 37, 49, 59, 67, 75, 85, 90];
    for &idx in &sample_indices {
        if let Some(email) = emails.get(idx) {
            let processed = engine.process(email);
            println!(
                "[{}] {:<13} | Preview: \"{}\"",
                processed.id,
                format!("[{}]", processed.primary_category.as_str()),
                processed.preview
            );
        }
    }
    println!("---------------------------------\n");

    // Run Comprehensive Benchmark
    let runner = BenchmarkRunner::new(&engine, &emails);
    let report = runner.run_all();

    print_benchmark_report(&report);

    Ok(())
}
