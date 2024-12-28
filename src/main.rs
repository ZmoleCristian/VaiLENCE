use std::env;
use std::io::{self, BufRead};

mod processing;
mod request;
mod types;

use processing::process_chunk;
use types::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = parse_arguments(env::args().collect());
    config.api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    print_settings(&config);

    let mut buffer = Vec::new();

    if let Some(file_path) = &config.file_path {
        let file = std::fs::File::open(file_path)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            buffer.push(line?);
            if buffer.len() >= config.chunk_size {
                process_chunk(&config, &buffer)?;
                buffer.clear();
            }
        }
    } else {
        let stdin = io::stdin();
        let handle = stdin.lock();
        for line in handle.lines() {
            buffer.push(line?);
            if buffer.len() >= config.chunk_size {
                process_chunk(&config, &buffer)?;
                buffer.clear();
            }
        }
    }

    if !buffer.is_empty() {
        process_chunk(&config, &buffer)?;
    }

    Ok(())
}

fn parse_arguments(args: Vec<String>) -> Config {
    let mut config = Config::default();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-s" | "--severity-min" => {
                if let Some(value) = args.get(i + 1) {
                    config.severity_min = value.parse().expect("Invalid severity-min value");
                    i += 1;
                }
            }
            "-i" | "--input" => {
                if let Some(value) = args.get(i + 1) {
                    config.file_path = Some(value.clone());
                    i += 1;
                }
            }
            "-o" | "--output" => {
                if let Some(value) = args.get(i + 1) {
                    config.output_file = Some(value.clone());
                    i += 1;
                }
            }
            "-v" | "--verbose" => config.verbose = true,
            "-c" | "--chunk-size" => {
                if let Some(value) = args.get(i + 1) {
                    config.chunk_size = value.parse().expect("Invalid chunk-size value");
                    i += 1;
                }
            }
            "-e" | "--error-retry" => {
                if let Some(value) = args.get(i + 1) {
                    config.error_retry = value.parse().expect("Invalid error-retry value");
                    i += 1;
                }
            }
            "-l" | "--loop" => config.loop_mode = true,
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            _ => {}
        }
        i += 1;
    }

    config
}

fn print_help() {
    println!("Usage: VaiLENCE [OPTIONS]");
    println!("Options:");
    println!("  -s, --severity-min <SEVERITY>  Sets the minimum severity threshold (default: 0.01)");
    println!("  -i, --file <FILE>              Path to file to process (default: stdin)");
    println!("  -o, --output <FILE>            Path to output JSON file (default: stdout)");
    println!("  -v, --verbose                  Prints output to stdout even if -o is specified (default: false)");
    println!("  -c, --chunk-size <CHUNK_SIZE>  Chunk size to process in one API call (default: 100)");
    println!("  -e, --error-retry <RETRY>      Number of times to retry on error (default: 3)");
    println!("  -l, --loop                     Run in continuous mode processing stdin indefinitely");
    println!("  -h, --help                     Print this help message");
}

fn print_settings(config: &Config) {
    println!("Program Settings:");
    println!("  - Minimum Severity: {:.2}", config.severity_min);
    println!("  - Input Source: {}", config.file_path.clone().unwrap_or_else(|| "stdin".to_string()));
    println!("  - Chunk Size: {} lines", config.chunk_size);
    println!("  - Error Retry Count: {}", config.error_retry);
    println!("  - Output File: {}", config.output_file.clone().unwrap_or_else(|| "stdout".to_string()));
    println!("  - Continuous Loop Mode: {}", config.loop_mode);
    println!("  - Print to Stdout: {}", if config.output_file.is_some() { config.verbose } else { true });
}
