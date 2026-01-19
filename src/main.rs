

use clap::Parser;
use skim::prelude::*;
use std::io::Cursor;
use std::process;

mod config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The key of the URL to open
    key: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let config = match config::load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            process::exit(1);
        }
    };

    if let Some(key) = cli.key {
        if let Some(url) = config.urls.get(&key) {
            open_url(url);
        } else {
            eprintln!("Key '{}' not found.", key);
            process::exit(1);
        }
    } else {
        // Fuzzy Finder
        let mut keys: Vec<String> = config.urls.keys().cloned().collect();
        if keys.is_empty() {
            println!("No URLs found in configuration.");
            return;
        }
        keys.sort(); // Sort keys for better UX

        // Prepare input for skim (newline separated)
        let input = keys.join("\n");

        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .multi(false)
            .build()
            .unwrap();

        let item_reader = SkimItemReader::default();
        let items = item_reader.of_bufread(Cursor::new(input));

        let selected = Skim::run_with(&options, Some(items))
            .map(|out| out.selected_items)
            .unwrap_or_else(Vec::new);

        if let Some(item) = selected.first() {
            let key = item.output();
            if let Some(url) = config.urls.get(key.as_ref()) {
                open_url(url);
            }
        }
    }
}

fn open_url(url: &str) {
    println!("Opening: {}", url);
    if let Err(e) = webbrowser::open(url) {
        eprintln!("Failed to open URL '{}': {}", url, e);
    }
}
