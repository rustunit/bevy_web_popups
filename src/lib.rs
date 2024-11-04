mod plugin;

#[cfg(target_family = "wasm")]
mod channel;
#[cfg(target_family = "wasm")]
mod web;

pub use plugin::{WebAlertResponse, WebAlertsPlugin};

/// Show a textinput dialog using DOM elements and communicate back to the bevy side via Events `WebAlertResponse`.
///
/// It will spawn the following DOM structure at the root of `<body>`:
///
/// ```html
/// <div class="bevy_wasm_popup_root">
///     <div class="bevy_wasm_popup_popup">
///         <strong class="bevy_wasm_popup_label">title</strong>
///         <input type="text" class="bevy_wasm_popup_text">
///         <div class="bevy_wasm_popup_buttons">
///             <button class="bevy_wasm_popup_button">button_label_cancel</button>
///             <button class="bevy_wasm_popup_button">button_label_ok</button>
///         </div>
///     </div>
/// </div>
/// ```
///
/// It will also register the button on click handlers.
/// After either button is pressed it will remove all these DOM elements again and send the last value of the input field back to us.
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

/// show the browsers standard `window.alert()` using the provided `msg`.
#[allow(clippy::missing_const_for_fn, unused_variables)]
pub fn alert(msg: &str) {
    #[cfg(target_family = "wasm")]
    {
        web::alert(msg);
    }
}
