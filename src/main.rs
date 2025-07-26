// TODO: use tracing

use clap::Parser;
use std::io::BufRead;
use tracing::Level;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    pub verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Set up logging
    // TODO: Maelstrom expects debug info like logs to be written to stderr.
    let mut filter = EnvFilter::from_default_env();

    if args.verbose {
        filter = filter.add_directive("debug".parse().unwrap());
    }

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    // Incoming messages are received as JSON values from stdin. Each message is
    // separated by a newline.
    eprintln!("gossip-glomer node started - type 'q' or 'quit' to exit");

    let mut line_buffer = String::new();

    loop {
        // Read the next message from stdin.
        let bytes_read = std::io::stdin().lock().read_line(&mut line_buffer)?;
        let line = line_buffer.trim();

        tracing::debug!("read {} bytes from stdin: `{}`", bytes_read, line);

        // Quit if there are no more bytes to read from stdin. This happens when
        // the host application (Maelstrom) tries to terminate the node by
        // closing stdin.
        if bytes_read == 0 {
            tracing::info!("EOF received, exiting...");
            break;
        }

        // Did the user ask to quit?
        if line == "q" || line == "quit" {
            tracing::info!("quit received, exiting...");
            break;
        }

        // Process the input as a JSON message.

        // Clear the input before reading the next line from stdin.
        line_buffer.clear();
    }

    Ok(())
}
