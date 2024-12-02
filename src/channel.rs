use bevy_channel_trigger::ChannelSender;
use std::sync::OnceLock;

use crate::plugin::WebAlertResponse;

static SENDER: OnceLock<Option<ChannelSender<WebAlertResponse>>> = OnceLock::new();

pub fn send_event(e: WebAlertResponse) {
    let Some(sender) = SENDER.get().map(Option::as_ref).flatten() else {
        return bevy::log::error!("`WebAlertsPlugin` not installed correctly (no sender found)");
    };
    sender.send(e);
}

pub fn set_sender(sender: ChannelSender<WebAlertResponse>) {
    while SENDER.set(Some(sender.clone())).is_err() {}
}
