use serde::{Deserialize, Serialize};

// TODO: serialization
// TODO: add unit tests for these methods.

#[derive(Serialize, Deserialize, Debug)]
pub struct MaelstromMessage<T> {
    src: String,
    dest: String,
    body: BodyWithTypedData<T>,
}

impl<T> MaelstromMessage<T> {
    pub fn new(src: String, dest: String, msg_id: Option<usize>, body: T) -> Self {
        MaelstromMessage {
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
        MaelstromMessage {
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

#[derive(Serialize, Deserialize, Debug)]
struct BodyWithTypedData<T> {
    #[serde(flatten)]
    typed: T,
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
}
