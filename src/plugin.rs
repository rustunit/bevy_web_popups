use bevy::prelude::*;

#[derive(Event, Clone, Debug)]
pub enum WebAlertResponse {
    InputOk(String),
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
                .world
                .get_resource::<CrossbeamEventSender<WebAlertResponse>>()
                .unwrap()
                .clone();

            crate::channel::set_sender(sender);
        }
    }
}
