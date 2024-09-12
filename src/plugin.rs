use bevy::prelude::*;

/// Events bridging communication back from the web to bevy
#[derive(Event, Clone, Debug)]
pub enum WebAlertResponse {
    /// A call to `show_textinput` returned with the user clicking the ok button. Contains text of input field.
    InputOk(String),
    /// A call to `show_textinput` returned with the user clicking the cancel button. Contains text of input field.
    InputCancel(String),
}

/// Bevy plugin to register the required machinery to receive events from web (on wasm) and register the `WebAlertResponse` event with bevy.
pub struct WebAlertsPlugin;
impl Plugin for WebAlertsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_family = "wasm"))]
        {
            app.add_event::<WebAlertResponse>();
        }

        #[cfg(target_family = "wasm")]
        {
            use bevy_crossbeam_event::{CrossbeamEventApp, CrossbeamEventSender};

            app.add_crossbeam_event::<WebAlertResponse>();

            let sender = app
                .world()
                .get_resource::<CrossbeamEventSender<WebAlertResponse>>()
                .unwrap()
                .clone();

            crate::channel::set_sender(sender);
        }
    }
}
