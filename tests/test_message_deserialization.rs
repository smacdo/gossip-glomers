mod messages;

use messages::TestNodeMessage;

use gossip_glomers::Message;

#[test]
fn parse_simple_maelstrom_message() {
    let message = r#"
    {
      "src": "alice",
      "dest": "bob",
      "body": {
        "type": "test",
        "msg_id": 345,
        "in_reply_to": 123 
      }
    }
    "#;
    let m: Message<TestNodeMessage> = serde_json::from_str(message).unwrap();

    assert_eq!(m.src(), "alice");
    assert_eq!(m.dest(), "bob");
    assert_eq!(m.msg_id(), Some(345));
    assert_eq!(m.in_reply_to(), Some(123));
    assert_eq!(m.body(), &TestNodeMessage::Test);
}

#[test]
fn parse_message_with_typed_fields() {
    let message = r#"
    {
      "src": "alice",
      "dest": "bob",
      "body": {
        "type": "test2",
        "msg_id": 345,
        "a": 3,
        "b": 22
      }
    }
    "#;
    let m: Message<TestNodeMessage> = serde_json::from_str(message).unwrap();

    assert_eq!(m.src(), "alice");
    assert_eq!(m.dest(), "bob");
    assert_eq!(m.msg_id(), Some(345));
    assert_eq!(m.body(), &TestNodeMessage::Test2 { a: 3, b: 22 });
}

#[test]
fn parse_message_with_string_typed_fields() {
    let message = r#"
    {
      "src": "alice",
      "dest": "bob",
      "body": {
        "type": "echo",
        "msg_id": 345,
        "message": "Hello World"
      }
    }
    "#;
    let m: Message<TestNodeMessage> = serde_json::from_str(message).unwrap();

    assert_eq!(m.src(), "alice");
    assert_eq!(m.dest(), "bob");
    assert_eq!(m.msg_id(), Some(345));
    assert_eq!(
        m.body(),
        &TestNodeMessage::Echo {
            message: "Hello World".to_string()
        }
    );
}

#[test]
fn parse_init_message() {
    let message = r#"
    {
      "src": "alice",
      "dest": "bob",
      "body": {
        "type": "init",
        "msg_id": 1,
        "node_id": "n3",
        "node_ids": ["n1", "n2", "n3"]
      }
    }
    "#;
    let m: Message<TestNodeMessage> = serde_json::from_str(message).unwrap();

    assert_eq!(m.src(), "alice");
    assert_eq!(m.dest(), "bob");
    assert_eq!(m.msg_id(), Some(1));
    assert_eq!(
        m.body(),
        &TestNodeMessage::Init {
            node_id: "n3".to_string(),
            node_ids: vec!["n1".to_string(), "n2".to_string(), "n3".to_string()]
        }
    );
}
