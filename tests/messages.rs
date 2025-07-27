use gossip_glomers::NodeMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Messages {
    Test,
    Test2 {
        a: usize,
        b: usize,
    },
    Echo {
        message: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
}

impl NodeMessage for Messages {}
