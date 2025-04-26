use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

/// Events bridging communication back from the web to bevy
#[derive(Event, Clone, Debug)]
pub enum WebAlertResponse {
    /// A call to `show_textinput` returned with the user clicking the ok button. Contains text of input field.
    InputOk(String),
    /// A call to `show_textinput` returned with the user clicking the cancel button. Contains text of input field.
    InputCancel(String),
}

/// Bevy plugin to register the required machinery to receive events from web (on wasm)
/// and register the `WebAlertResponse` event with bevy.
/// This turns to a no-op on anything non-wasm, so it does not need to be cfg-gated.
pub struct WebAlertsPlugin;
impl Plugin for WebAlertsPlugin {
    #[cfg_attr(not(target_family = "wasm"), allow(unused_variables))]
    fn build(&self, app: &mut App) {
        #[cfg(target_family = "wasm")]
        {
            use bevy_channel_trigger::ChannelTriggerApp;

            let sender = app.add_channel_trigger::<WebAlertResponse>();

            crate::channel::set_sender(sender);
        }
    }
}
