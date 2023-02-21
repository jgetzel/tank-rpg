use std::cmp::max;
use bevy::prelude::Resource;
use crate::networking::client::RequestId;

#[derive(Resource, Default)]
pub struct RequestIdCounter {
    last_sent_id: RequestId,
    last_ack_id: RequestId,
}

impl RequestIdCounter {
    pub fn next_id(&mut self) -> RequestId {
        self.last_sent_id += 1;
        self.last_sent_id
    }

    pub fn last_sent_id(&self) -> RequestId {
        self.last_sent_id
    }

    pub fn last_ack_id(&self) -> RequestId {
        self.last_ack_id
    }

    pub fn update_ack(&mut self, id: RequestId) -> bool {
        self.last_ack_id = max(id, self.last_ack_id);
        if id > self.last_ack_id {
            self.last_ack_id = id;
            true
        }
        else {
            false
        }
    }
}