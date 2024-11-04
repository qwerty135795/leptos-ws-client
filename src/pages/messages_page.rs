use leptos::{component, create_effect, create_signal, event_target_value, view, IntoView, SignalGet};
use web_sys::Event;
use crate::js;
#[component]
pub fn messages() -> impl IntoView {
    create_effect(move |_| {
       js::connectToSocketIO("127.0.0.1:5000");
    });
    view! {
            <div className="d-flex flex-column bg-light" style="width:250px; height:100vh">
      <SearchBar />
    </div>
    }
}
#[component]
fn search_bar() -> impl IntoView {
    let (search_term, set_search_term) = create_signal("".to_owned());
    let search = {
        move |ev:Event| {
            let value = event_target_value(&ev);
            leptos::tracing::info!("value is {}", value);
            set_search_term(value);
        }
    };
    create_effect(move |_| {
        let value = search_term.get();
        if value.len() > 3 {
            js::search_users(&value)
        };
    });
    view! {
            <div className="p-3">
      <div className="input-group">
        <input
          type="text"
          className="form-control"
          placeholder="username aks 'astra'"
          prop:value=search_term
            on:input=move |ev| {
            search(ev)
        }
        />
        <button className="btn btn-outline-secondary" type="button">
          <i className="bi bi-search"></i> {/* Иконка поиска */}
        </button>
      </div>
    </div>
    }
}