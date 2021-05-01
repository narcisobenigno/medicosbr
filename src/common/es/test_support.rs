use crate::common::es::Payload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct TestEvent {
    pub name: String,
    pub content: String,
}

impl Payload for TestEvent {
    type UnmarshalErr = ();

    fn name(&self) -> String {
        self.name.clone()
    }

    fn marshal_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn unmarshal_json(payload: &str) -> Result<Self, Self::UnmarshalErr> {
        match serde_json::from_str::<Self>(payload) {
            Ok(payload) => Ok(payload),
            _ => Err(()),
        }
    }
}
