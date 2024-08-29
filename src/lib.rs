mod plugin;

#[cfg(target_family = "wasm")]
mod channel;
#[cfg(target_family = "wasm")]
mod web;

pub use plugin::{WebAlertResponse, WebAlertsPlugin};

#[allow(clippy::missing_const_for_fn, unused_variables)]
pub fn show_textinput(
    title: &str,
    button_label_ok: &str,
    button_label_cancel: &str,
    create_styles: bool,
) {
    #[cfg(target_family = "wasm")]
    {
        web::show_textinput(title, button_label_ok, button_label_cancel, create_styles);
    }
}

#[allow(clippy::missing_const_for_fn, unused_variables)]
pub fn alert(msg: &str) {
    #[cfg(target_family = "wasm")]
    {
        web::alert(msg);
    }
}
