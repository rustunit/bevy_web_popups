use bevy_channel_trigger::ChannelSender;
use std::sync::OnceLock;

use crate::plugin::WebAlertResponse;

static SENDER: OnceLock<Option<ChannelSender<WebAlertResponse>>> = OnceLock::new();

//TODO: error logging
pub fn send_event(e: WebAlertResponse) {
    SENDER
        .get()
        .expect("invalid sender lock")
        .as_ref()
        .expect("sender not found")
        .send(e);
}

pub fn set_sender(sender: ChannelSender<WebAlertResponse>) {
    while SENDER.set(Some(sender.clone())).is_err() {}
}
