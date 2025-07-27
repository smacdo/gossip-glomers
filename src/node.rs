use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{
    NodeMessage,
    io::{MessageReader, MessageWriter},
};

// TODO: make an injected custom type.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum XYZNodeMessage {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
}

impl NodeMessage for XYZNodeMessage {}

pub struct Node<T, R, W>
where
    T: NodeMessage,
    R: MessageReader<T>,
    W: MessageWriter<T>,
{
    reader: R,
    writer: W,
    _phantom: PhantomData<T>,
}

/*
impl<T, R, W> Node<T, R, W>
where
    R: MessageReader<T>,
    W: MessageWriter<T>,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader,
            writer,
            _phantom: PhantomData,
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        // TODO: handle errors
        loop {
            let message = self.reader.read()?;
        }
    }
}
    */
