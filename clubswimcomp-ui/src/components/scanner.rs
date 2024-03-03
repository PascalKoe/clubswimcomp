use leptos::*;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/scanner.js")]
extern "C" {
    fn init_scanner(connected_id: String, value_id: String);
}

#[component]
pub fn Scanner(
    #[prop(into)] value_scanned: Callback<String>,
) -> impl IntoView {
    let scanner_id = Uuid::new_v4();
    let connected_id = move || format!("scanner-connected-{scanner_id}");
    let value_id = move || format!("scanner-value-{scanner_id}");

    let connected = create_rw_signal(false);

    let on_connect_clicked = move |_| {
        init_scanner(connected_id(), value_id());
    };

    let connect_button_text = move || match connected() {
        true => "Connected",
        false => "Connect Scanner",
    };

    let connected_node = create_node_ref();
    let _ = leptos_use::use_event_listener(connected_node, ev::input, move |ev| {
        let value = event_target_value(&ev);
        let value = match value.as_str() {
            "connected" => true,
            _ => false,
        };

        connected.set(value);
    });

    let value_node = create_node_ref();
    let _ = leptos_use::use_event_listener(value_node, ev::input, move |ev| {
        let value = event_target_value(&ev);
        value_scanned(value);
    });

    view! {
        <button class="btn btn-sm btn-primary rounded-full mr-4" on:click=on_connect_clicked disabled=connected>
            <phosphor_leptos::QrCode />
            {connect_button_text}
        </button>
        <input id=connected_id node_ref=connected_node hidden />
        <input id=value_id node_ref=value_node hidden />
    }
}
