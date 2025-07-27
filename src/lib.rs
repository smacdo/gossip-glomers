pub mod io;
pub mod node;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

// TODO: serialization
// TODO: add unit tests for these methods.

pub trait NodeMessage: Debug + PartialEq /* + Send + Sync */ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Message<T: NodeMessage> {
    src: String,
    dest: String,
    body: BodyWithTypedData<T>,
}

impl<T> Message<T>
where
    T: NodeMessage,
{
    pub fn new(src: String, dest: String, msg_id: Option<usize>, body: T) -> Self {
        Message {
            src,
            dest,
            body: BodyWithTypedData {
                typed: body,
                msg_id,
                in_reply_to: None,
            },
        }
    }

    pub fn new_reply(
        src: String,
        dest: String,
        reply_to: usize,
        msg_id: Option<usize>,
        body: T,
    ) -> Self {
        Message {
            src,
            dest,
            body: BodyWithTypedData {
                typed: body,
                msg_id,
                in_reply_to: Some(reply_to),
            },
        }
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn dest(&self) -> &str {
        &self.dest
    }

    pub fn msg_id(&self) -> Option<usize> {
        self.body.msg_id
    }

    pub fn in_reply_to(&self) -> Option<usize> {
        self.body.in_reply_to
    }

    pub fn body(&self) -> &T {
        &self.body.typed
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct BodyWithTypedData<T>
where
    T: NodeMessage,
{
    #[serde(flatten)]
    typed: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<usize>,
}
