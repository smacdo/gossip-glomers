// TODO: use tracing

use clap::Parser;
use gossip_glomers::{
    io::{MessageReader, MessageReaderError, StdinMessageReader},
    node::XYZNodeMessage,
};
use tracing_subscriber::EnvFilter;

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
    } else {
        filter = filter.add_directive("info".parse().unwrap());
    }

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    // Incoming messages are received as JSON values from stdin. Each message is
    // separated by a newline.
    tracing::info!("gossip-glomer node started - type 'q' or 'quit' to exit");
    let reader: StdinMessageReader<XYZNodeMessage> = StdinMessageReader::new();

    loop {
        match reader.read() {
            Ok(message) => {
                // TODO: handle the message.
                tracing::debug!("GOT: {message:?}");
            }
            Err(MessageReaderError::Closed) | Err(MessageReaderError::Quit) => {
                break;
            }
            Err(e) => {
                tracing::error!("{e}");
            }
        }
    }

    tracing::info!("gossip-glomer node stopped");

    Ok(())
}
