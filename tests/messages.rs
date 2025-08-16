use gossip_glomers::{
    NodeMessage,
    node::{CreatesInitOkMessage, InitMessage, TryIntoInitMessage},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum TestNodeMessage {
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
    InitOk,
    Quit,
}

impl NodeMessage for TestNodeMessage {}

impl TryIntoInitMessage for TestNodeMessage {
    fn try_into_init_message(&self) -> Option<InitMessage> {
        match self {
            TestNodeMessage::Init { node_id, node_ids } => Some(InitMessage {
                node_id: node_id.clone(),
                node_ids: node_ids.clone(),
            }),
            _ => None,
        }
    }
}

impl CreatesInitOkMessage for TestNodeMessage {
    fn create_init_ok() -> Self {
        TestNodeMessage::InitOk
    }
}
