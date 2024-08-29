use gloo::events::EventListener;
use web_sys::{wasm_bindgen::JsCast, Element, HtmlInputElement};

use crate::{channel::send_event, plugin::WebAlertResponse};

static STYLES: &str = r"
.bevy_wasm_popup_root { 
   position: absolute;
    width: 100%;
    height: 100%;
    background: #00000094;
    z-index: 10;

    display: flex;
    flex-direction: column;
    justify-content: center;
}

.bevy_wasm_popup_popup { 
    background-color: #ababab;
    margin-right: auto;
    margin-left: auto;
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 12px;
    border-radius: 15px;
}

.bevy_wasm_popup_text { 
    margin-left: auto;
    margin-right: auto;
    border-radius: 5px;
    border-style: solid;
    border-width: 1px;
    border-color: #818181;
}

.bevy_wasm_popup_buttons { 
    display: flex;
    flex-direction: row;
    padding: 0;
    margin: 0;
    gap: 0;
}

.bevy_wasm_popup_button { 
    border-radius: 5px;
    border-style: solid;
    border-width: 1px;
    border-color: #818181;
    flex: 1 1 0px;
}

.bevy_wasm_popup_label { 
    font-weight: bold;
    font-size: 14px;
}
";

pub fn show_textinput(
    title: &str,
    button_label_ok: &str,
    button_label_cancel: &str,
    create_styles: bool,
) {
    let doc = gloo::utils::document();

    let root = doc.create_element("div").unwrap();
    gloo::utils::body().append_child(&root).unwrap();

    let popup = doc.create_element("div").unwrap();
    root.append_child(&popup).unwrap();

    let label = doc.create_element("strong").unwrap();
    label.set_text_content(Some(title));
    popup.append_child(&label).unwrap();

    let textinput = doc.create_element("input").unwrap();
    textinput.set_attribute("type", "text").unwrap();
    popup.append_child(&textinput).unwrap();

    let button_row = doc.create_element("div").unwrap();
    popup.append_child(&button_row).unwrap();

    let button_cancel = doc.create_element("button").unwrap();
    button_cancel.set_text_content(Some(button_label_cancel));
    button_row.append_child(&button_cancel).unwrap();

    let button_ok = doc.create_element("button").unwrap();
    button_ok.set_text_content(Some(button_label_ok));
    button_row.append_child(&button_ok).unwrap();

    root.set_class_name("bevy_wasm_popup_root");
    popup.set_class_name("bevy_wasm_popup_popup");
    label.set_class_name("bevy_wasm_popup_label");
    textinput.set_class_name("bevy_wasm_popup_text");
    button_row.set_class_name("bevy_wasm_popup_buttons");
    button_ok.set_class_name("bevy_wasm_popup_button");
    button_cancel.set_class_name("bevy_wasm_popup_button");

    if create_styles {
        create_style(STYLES).expect("could not create style");
    }

    {
        let root = root.clone();
        let textinput: HtmlInputElement = textinput.clone().dyn_into().expect("wrong type");
        EventListener::new(&button_ok, "click", move |_event| {
            send_event(WebAlertResponse::InputOk(textinput.value()));
            root.remove();
        })
        .forget();
    }

    {
        let root = root.clone();
        let textinput: HtmlInputElement = textinput.clone().dyn_into().expect("wrong type");
        EventListener::new(&button_cancel, "click", move |_event| {
            send_event(WebAlertResponse::InputCancel(textinput.value()));
            root.remove();
        })
        .forget();
    }
}

fn create_style(css: &str) -> Option<Element> {
    let style = gloo::utils::document().create_element("style").ok()?;

    style.set_attribute("type", "text/css").ok()?;
    style.set_inner_html(css);
    gloo::utils::head().append_child(&style).ok()?;

    Some(style)
}

pub fn alert(msg: &str) {
    gloo::utils::window().alert_with_message(msg).expect("");
}
