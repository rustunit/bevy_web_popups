use bevy_crossbeam_event::CrossbeamEventSender;
use std::sync::OnceLock;

use crate::plugin::WebAlertResponse;

static SENDER: OnceLock<Option<CrossbeamEventSender<WebAlertResponse>>> = OnceLock::new();

//TODO: error logging
pub fn send_event(e: WebAlertResponse) {
    SENDER
        .get()
        .expect("invalid sender lock")
        .as_ref()
        .expect("sender not found")
        .send(e);
}

pub fn set_sender(sender: CrossbeamEventSender<WebAlertResponse>) {
    while SENDER.set(Some(sender.clone())).is_err() {}
}
