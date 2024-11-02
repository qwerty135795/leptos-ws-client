use leptos::{component, view, IntoView};
use crate::js;
#[component]
pub fn messages() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            js::connectToSocketIO("127.0.0.1:5000/")
        }
        >
        "Try connect"
        </button>
    }
}