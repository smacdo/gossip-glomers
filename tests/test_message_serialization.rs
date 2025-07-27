mod messages;

use messages::Messages;

use gossip_glomers::Message;
use serde_json::{Value, json};

#[test]
fn serialize_test_maelstrom_message() {
    let og_message = Message::new(
        "alice".to_string(),
        "bob".to_string(),
        Some(10),
        Messages::Test2 { a: 42, b: 22 },
    );

    let s = serde_json::to_string(&og_message).unwrap();
    let json_obj: Value = serde_json::from_str(&s).unwrap();

    assert_eq!(
        json_obj,
        json!(
            {
                "src": "alice",
                "dest": "bob",
                "body": {
                    "type": "test2",
                    "msg_id": 10,
                    "a": 42,
                    "b": 22
                }
            }
        )
    );
}

#[test]
fn serialize_message_with_reply_to() {
    let og_message = Message::new_reply(
        "alice".to_string(),
        "bob".to_string(),
        7,
        Some(10),
        Messages::Test,
    );

    let s = serde_json::to_string(&og_message).unwrap();
    let json_obj: Value = serde_json::from_str(&s).unwrap();

    assert_eq!(
        json_obj,
        json!(
            {
                "src": "alice",
                "dest": "bob",
                "body": {
                    "type": "test",
                    "msg_id": 10,
                    "in_reply_to": 7
                }
            }
        )
    );
}

#[test]
fn serialize_message_with_no_id_or_reply_to() {
    let og_message = Message::new("alice".to_string(), "bob".to_string(), None, Messages::Test);

    let s = serde_json::to_string(&og_message).unwrap();
    let json_obj: Value = serde_json::from_str(&s).unwrap();

    assert_eq!(
        json_obj,
        json!(
            {
                "src": "alice",
                "dest": "bob",
                "body": {
                    "type": "test"
                }
            }
        )
    );
}
