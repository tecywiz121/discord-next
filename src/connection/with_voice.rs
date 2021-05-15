use crate::voice::VoiceConnection;

use std::collections::HashMap;

use super::*;

pub struct Voice {
    pub voice_handles: HashMap<Option<ServerId>, VoiceConnection>,
    pub user_id: UserId,
}

impl Voice {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            voice_handles: Default::default(),
        }
    }
}
