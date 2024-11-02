mod models;
mod pages;
mod js;

use std::sync::Arc;
use leptos::{component, create_resource, event_target_value, mount_to_body, spawn_local, view, IntoView};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use std::option::Option;
use leptos::svg::path;
use leptos_router::{Route, Router, Routes};
use crate::models::{LoginModel, RegisterModel, Tokens};
use leptos_router::*;
use pages::{auth_page::Auth, messages_page::Messages};
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
#[component]
fn App() -> impl IntoView {
    let (tokens, set_tokens) = create_signal::<Option<Tokens>>(None);
    let (tokens, set_tokens) =
        (tokens.clone(),set_tokens.clone());
    let only_auth = {
        move || {
            if let Some(_) = tokens() {
                view!{<Messages />}
            } else {
                view!{ <Auth prop_callback=set_tokens  />}
            }
        }
    };
    view! {
        <div style="display:flex; flex-direction:column;height:100vh; wight:100%;">
            <Header />
            <div style="flex: 1; display: flex;">
                 <Router>
                    <Routes>
                    <Route path="/" view=move || only_auth/>

                 </Routes>
                 </Router>
            </div>
        </div>
    }
}

#[component]
fn header() -> impl IntoView {
    view! {
        <header class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container-fluid">
                <a class="navbar-brand" href="#">
                    {"RustChat"}
                </a>

                <button
                    class="navbar-toggler"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarContent"
                    aria-controls="navbarContent"
                    aria-expanded="false"
                    aria-label="Toggle navigation"
                >
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarContent">
                    <ul class="navbar-nav ms-auto mb-2 mb-lg-0">
                        <li class="nav-item">
                            <a class="nav-link" href="#">
                                {"Rooms"}
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#" aria-label="Notifications">
                                <i class="bi bi-bell"></i>
                            </a>
                        </li>
                        <li class="nav-item dropdown">
                            <a
                                class="nav-link dropdown-toggle"
                                href="#"
                                id="userDropdown"
                                role="button"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                <img
                                    src="path_to_profile_image.jpg"
                                    alt="Profile"
                                    class="rounded-circle"
                                    width="30"
                                    height="30"
                                />
                            </a>
                            <ul
                                class="dropdown-menu dropdown-menu-end"
                                aria-labelledby="userDropdown"
                            >
                                <li>
                                    <a class="dropdown-item" href="#">
                                        {"Profile"}
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#">
                                        {"Settings"}
                                    </a>
                                </li>
                                <li>
                                    <hr class="dropdown-divider" />
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#">
                                        {"Logout"}
                                    </a>
                                </li>
                            </ul>
                        </li>
                    </ul>
                </div>
            </div>
        </header>
    }
}