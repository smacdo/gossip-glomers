use std::marker::PhantomData;

use thiserror::Error;

use crate::{
    Message, NodeMessage,
    io::{MessageReader, MessageReaderError, MessageWriter, MessageWriterError},
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
    node_id: Option<String>,
    all_node_ids: Option<Vec<String>>,
    next_message_id: usize,
}

impl<T, R, W> Node<T, R, W>
where
    T: NodeMessage + TryIntoInitMessage + CreatesInitOkMessage,
    R: MessageReader<T>,
    W: MessageWriter<T>,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader,
            writer,
            _phantom: PhantomData,
            node_id: None,
            all_node_ids: None,
            next_message_id: 1,
        }
    }

    pub fn node_id(&self) -> Option<&str> {
        self.node_id.as_deref()
    }

    pub fn all_node_ids(&self) -> Option<Vec<String>> {
        self.all_node_ids.clone()
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
                if let Some(InitMessage { node_id, node_ids }) =
                    message.body().try_into_init_message()
                {
                    self.handle_init_message(message, node_id, node_ids)
                } else {
                    self.handle_user_message(message)
                }
            }
            Err(MessageReaderError::Closed) | Err(MessageReaderError::Quit) => Ok(false),
            Err(e) => {
                //                tracing::error!("{e}");
                Err(NodeError::MessageReader(e))
            }
        }
    }

    fn handle_init_message(
        &mut self,
        message: Message<T>,
        node_id: String,
        all_node_ids: Vec<String>,
    ) -> Result<bool, NodeError> {
        self.node_id = Some(node_id);
        self.all_node_ids = Some(all_node_ids);

        tracing::info!("Node id will be set to {}", self.node_id.as_ref().unwrap());
        tracing::info!(
            "Nodes in cluster are {}",
            self.all_node_ids.as_ref().unwrap().as_slice().join(",")
        );

        let ok_message: Message<_> = Message::new_reply(
            message.dest().to_string(),
            message.src().to_string(),
            message.msg_id().expect("requests must have msg id"),
            self.allocate_next_message_id(),
            T::create_init_ok(),
        );

        self.writer.write(ok_message)?;

        Ok(true)
    }

    fn handle_user_message(&mut self, message: Message<T>) -> Result<bool, NodeError> {
        tracing::info!("USER MSG: {message:?}");
        Ok(true)
    }

    fn allocate_next_message_id(&mut self) -> usize {
        let next_id = self.next_message_id;
        self.next_message_id += 1;
        next_id
    }
}

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("{}", .0)]
    MessageReader(#[from] MessageReaderError),
    #[error("{}", .0)]
    MessageWriter(#[from] MessageWriterError),
}

pub struct InitMessage {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

pub trait TryIntoInitMessage {
    fn try_into_init_message(&self) -> Option<InitMessage>;
}

pub trait CreatesInitOkMessage {
    fn create_init_ok() -> Self;
}
