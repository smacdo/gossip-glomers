mod messages;

use std::collections::VecDeque;

use gossip_glomers::{
    Message, NodeMessage,
    io::{MessageReader, MessageReaderError, MessageWriter, MessageWriterError},
    node::Node,
};

use crate::messages::TestNodeMessage;

struct TestableMessageReader<'a> {
    incoming_messages: &'a mut VecDeque<Message<TestNodeMessage>>,
}

impl<'a> TestableMessageReader<'a> {
    pub fn new(incoming_messages: &'a mut VecDeque<Message<TestNodeMessage>>) -> Self {
        Self { incoming_messages }
    }
}

impl<'a> MessageReader<TestNodeMessage> for TestableMessageReader<'a> {
    fn read(&mut self) -> Result<Message<TestNodeMessage>, MessageReaderError> {
        let message = self.incoming_messages.pop_front().unwrap();

        match message.body() {
            TestNodeMessage::Quit => Err(MessageReaderError::Quit),
            _ => Ok(message),
        }
    }
}

struct TestableMessageWriter<'a, T: NodeMessage> {
    outgoing_messages: &'a mut VecDeque<Message<T>>,
}

impl<'a, T: NodeMessage> TestableMessageWriter<'a, T> {
    pub fn new(outgoing_messages: &'a mut VecDeque<Message<T>>) -> Self {
        Self { outgoing_messages }
    }
}

impl<'a, T: NodeMessage> MessageWriter<T> for TestableMessageWriter<'a, T> {
    fn write(&mut self, message: Message<T>) -> Result<(), MessageWriterError> {
        self.outgoing_messages.push_back(message);
        Ok(())
    }
}

#[test]
fn node_initializes_when_sent_init_message() {
    let mut incoming: VecDeque<Message<TestNodeMessage>> = VecDeque::new();
    let mut outgoing: VecDeque<Message<TestNodeMessage>> = VecDeque::new();

    // TODO: Move this after node new, to allow for dynamic messaging
    incoming.push_back(Message::new(
        "c1".to_string(),
        "n2".to_string(),
        Some(1),
        TestNodeMessage::Init {
            node_id: "n2".to_string(),
            node_ids: vec!["n1".to_string(), "n2".to_string(), "n3".to_string()],
        },
    ));

    incoming.push_back(Message::new(
        "c1".to_string(),
        "n2".to_string(),
        Some(2),
        TestNodeMessage::Quit,
    ));

    let mut node: Node<TestNodeMessage, _, _> = Node::new(
        TestableMessageReader::new(&mut incoming),
        TestableMessageWriter::new(&mut outgoing),
    );

    node.run().expect("node must execute without errors");

    assert_eq!(node.node_id(), Some("n2"));
    assert_eq!(
        node.all_node_ids(),
        Some(vec!["n1".to_string(), "n2".to_string(), "n3".to_string()])
    );

    //assert_eq!(incoming.len(), 0);
}
