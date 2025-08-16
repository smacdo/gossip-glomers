use std::{
    cell::RefCell,
    io::{BufRead, Write},
    marker::PhantomData,
};

use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::{Message, NodeMessage};

#[derive(Error, Debug)]
pub enum MessageReaderError {
    #[error("the input source was closed by the host")]
    Closed,
    #[error("quit request received")]
    Quit,
    #[error("{}", .0)]
    Io(#[from] std::io::Error),
    #[error("{}", .0)]
    Deserialization(#[from] serde_json::Error),
}

/// Reads messages from an input source such as stdin or a network socket.
pub trait MessageReader<T>
where
    T: NodeMessage,
{
    /// Returns the next unread message from the reader. This method will block
    /// until a message is available.
    fn read(&mut self) -> Result<Message<T>, MessageReaderError>;
}

pub struct StdinMessageReader<T>
where
    T: DeserializeOwned,
{
    line_buffer: RefCell<String>,
    _phantom: PhantomData<T>,
}

impl<T> StdinMessageReader<T>
where
    T: DeserializeOwned,
{
    pub fn new() -> Self {
        Self {
            line_buffer: RefCell::new(String::new()),
            _phantom: Default::default(),
        }
    }
}

impl<T> MessageReader<T> for StdinMessageReader<T>
where
    T: NodeMessage + DeserializeOwned,
{
    fn read(&mut self) -> Result<Message<T>, MessageReaderError> {
        let mut line_buffer = self.line_buffer.borrow_mut();
        line_buffer.clear();

        // Read the next message from stdin.
        let bytes_read = std::io::stdin().lock().read_line(&mut line_buffer)?;
        let line = line_buffer.trim();

        tracing::debug!("read {} bytes from stdin: `{}`", bytes_read, line);

        // Quit if there are no more bytes to read from stdin. This happens when
        // the host application (Maelstrom) tries to terminate the node by
        // closing stdin.
        if bytes_read == 0 {
            tracing::info!("EOF received, exiting...");
            return Err(MessageReaderError::Closed);
        }

        // Did the user ask to quit?
        if line == "q" || line == "quit" {
            tracing::info!("read quit from stdin, will return Err(quit)");
            return Err(MessageReaderError::Quit);
        }

        // Process the input as a JSON message.
        let message: Message<T> = serde_json::from_str(&line_buffer)?;
        Ok(message)
    }
}

impl<T> Default for StdinMessageReader<T>
where
    T: DeserializeOwned,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum MessageWriterError {
    #[error("{}", .0)]
    Io(#[from] std::io::Error),
    #[error("{}", .0)]
    Serialization(#[from] serde_json::Error),
}

pub trait MessageWriter<T>
where
    T: NodeMessage,
{
    fn write(&mut self, message: Message<T>) -> Result<(), MessageWriterError>;
}

pub struct StdoutMessageWriter<T>
where
    T: serde::Serialize,
{
    _phantom: PhantomData<T>,
}

impl<T> StdoutMessageWriter<T>
where
    T: serde::Serialize,
{
    pub fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<T> Default for StdoutMessageWriter<T>
where
    T: serde::Serialize,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MessageWriter<T> for StdoutMessageWriter<T>
where
    T: NodeMessage + serde::Serialize,
{
    fn write(&mut self, message: Message<T>) -> Result<(), MessageWriterError> {
        let json_str = serde_json::to_string(&message)?;
        Ok(writeln!(std::io::stdout(), "{json_str}")?)
    }
}
