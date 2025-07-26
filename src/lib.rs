use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MaelstromMessage {
    src: String,
    dest: String,
    body: MessageBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageBody {
    #[serde(flatten)]
    typed: MessageTypes,
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum MessageTypes {
    Test,
    Read { key: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let m: MaelstromMessage = serde_json::from_str(message).unwrap();

        assert_eq!(&m.src, "alice");
        assert_eq!(&m.dest, "bob");
        assert_eq!(m.body.msg_id, Some(345));
        assert_eq!(m.body.in_reply_to, Some(123));
        assert_eq!(m.body.typed, MessageTypes::Test);
    }

    #[test]
    fn parse_message_with_typed_fields() {
        let message = r#"
    {
      "src": "alice",
      "dest": "bob",
      "body": {
        "type": "read",
        "msg_id": 345,
        "key": 3
      }
    }
    "#;
        let m: MaelstromMessage = serde_json::from_str(message).unwrap();

        assert_eq!(&m.src, "alice");
        assert_eq!(&m.dest, "bob");
        assert_eq!(m.body.msg_id, Some(345));
        assert_eq!(m.body.typed, MessageTypes::Read { key: 3 });
    }
}
