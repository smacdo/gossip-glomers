// TODO: use tracing

use clap::Parser;
use gossip_glomers::{
    NodeMessage,
    io::{StdinMessageReader, StdoutMessageWriter},
    node::Node,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum EchoNodeMessage {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
}

impl NodeMessage for EchoNodeMessage {}

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

    // Create a network node and run it.
    let mut node: Node<EchoNodeMessage, _, _> =
        Node::new(StdinMessageReader::new(), StdoutMessageWriter::new());
    Ok(node.run()?)
}
