use std::marker::PhantomData;

use thiserror::Error;

use crate::{
    NodeMessage,
    io::{MessageReader, MessageReaderError, MessageWriter},
};

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

impl<T, R, W> Node<T, R, W>
where
    T: NodeMessage,
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

    pub fn run(&mut self) -> Result<(), NodeError> {
        tracing::info!("gossip-glomer echo node started - type 'q' or 'quit' to exit");

        loop {
            match self.run_step() {
                Ok(true) => {}
                Ok(false) => {
                    tracing::debug!("received signal for node quit");
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

    pub fn run_step(&mut self) -> Result<bool, NodeError> {
        match self.reader.read() {
            Ok(message) => {
                tracing::info!("HANDLE MSG: {message:?}");
                Ok(true)
            }
            Err(MessageReaderError::Closed) | Err(MessageReaderError::Quit) => Ok(false),
            Err(e) => {
                //                tracing::error!("{e}");
                Err(NodeError::MessageReader(e))
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("{}", .0)]
    MessageReader(#[from] MessageReaderError),
}
