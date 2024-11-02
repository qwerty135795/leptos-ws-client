use gloo_storage::Storage;
use leptos::{component, create_resource, create_signal, event_target_value, spawn_local, view, IntoView, SignalGet, WriteSignal};

use crate::models::{LoginModel, RegisterModel, Tokens};

#[component]
pub fn Auth(prop_callback:WriteSignal<Option<Tokens>>) -> impl IntoView {
    view! {
        <div
            style="flex: 1; display: flex; background: radial-gradient(circle at center, #da3802, #000);"
            class="d-flex align-items-center justify-content-center m-0"
        >
            <div class="card p-4 shadow-sm" style="width: 100%; max-width: 400px;">
                <h3 class="text-center mb-4">{"Welcome to ChatApp"}</h3>
                <ul class="nav nav-tabs mb-3" id="authTabs" role="tablist">
                    <li class="nav-item" role="presentation">
                        <button
                            class="nav-link active"
                            id="login-tab"
                            data-bs-toggle="tab"
                            data-bs-target="#login"
                            type="button"
                            role="tab"
                            aria-controls="login"
                            aria-selected="true"
                        >
                            {"Login"}
                        </button>
                    </li>
                    <li class="nav-item" role="presentation">
                        <button
                            class="nav-link"
                            id="register-tab"
                            data-bs-toggle="tab"
                            data-bs-target="#register"
                            type="button"
                            role="tab"
                            aria-controls="register"
                            aria-selected="false"
                        >
                            {"Register"}
                        </button>
                    </li>
                </ul>
                <div class="tab-content" id="authTabsContent">
                    <LoginForm set_tokens=prop_callback />

                    <RegisterForm />
                </div>
            </div>
        </div>
    }
}
#[component]
fn login_form(set_tokens:WriteSignal<Option<Tokens>>) -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let set_tokens = set_tokens.clone();
    let submit = {
        move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            //leptos::logging::log!("username: {}, password: {}", login.username, login.password);
            let result = create_resource(|| (), move |_| {
                let login = LoginModel {
                    username: username.get(),
                    password: password.get()
                };
                async move {
                    //let body = serde_wasm_bindgen::to_value(&login).unwrap();
                    let res = reqwasm::http::Request::post("http://127.0.0.1:5000/auth")
                        .header("Content-Type","application/json")
                        .body(serde_json::to_string(&login).unwrap()).send().await
                        .unwrap();
                    let tokns = res.json::<Tokens>().await.ok();
                    if let Some(toks) = tokns {
                        gloo_storage::LocalStorage::set("tokens", &toks).unwrap_or_else(|err|{
                            println!("{err}")
                        });
                        set_tokens(Some(toks))
                    }
                }});
        }
    };
    view! {
        <div
            class="tab-pane fade show active"
            id="login"
            role="tabpanel"
            aria-labelledby="login-tab"
        >
            <form on:submit=submit>
                <div class="mb-3">
                    <label for="username" class="form-label">
                        {"Username"}
                    </label>
                    <input
                        type="text"
                        class="form-control"
                        id="username"
                        on:input = move |ev| {
            set_username(event_target_value(&ev));
        }
                        prop:value = username
                        placeholder="Enter your email"
                    />
                </div>
                <div class="mb-3">
                    <label for="loginPassword" class="form-label">
                        {"Password"}
                    </label>
                    <input
                        type="password"
                        class="form-control"
                        prop:value = password
                        on:input = move |ev| {
            set_password(event_target_value(&ev));
        }
                        id="loginPassword"
                        placeholder="Enter your password"
                    />
                </div>
                <button type="submit" class="btn btn-primary w-100">
                    {"Login"}
                </button>
            </form>
        </div>
    }
}
#[component]
fn register_form() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let send_form = {
        move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();

            spawn_local(async move {
                let register_model = RegisterModel {
                    username: username.get(),
                    email: email.get(),
                    password: password.get()
                };

                match reqwasm::http::Request::post("http://127.0.0.1:5000/user")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&register_model).unwrap())
                    .send().await {
                    Ok(res) => {
                        if res.ok() {
                            leptos::logging::log!("created");
                        } else {
                            leptos::logging::warn!("{}", res.status())
                        }
                    },
                    Err(err) => {
                        leptos::logging::error!("{err}");
                    }
                }
            })
        }
    };
    view! {
        <div class="tab-pane fade" id="register" role="tabpanel" aria-labelledby="register-tab">
            <form on:submit=send_form>
                <div class="mb-3">
                    <label for="registerUsername" class="form-label">
                        {"Username"}
                    </label>
                    <input
                        type="text"
                        class="form-control"
                        prop:value=username
                        on:input= move |ev| {
                        set_username(event_target_value(&ev));
        }
                        id="registerUsername"
                        placeholder="Choose a username"
                    />
                </div>
                <div class="mb-3">
                    <label for="registerEmail" class="form-label">
                        {"Email address"}
                    </label>
                    <input
                        type="email"
                        class="form-control"
                        prop:value=email
                        on:input= move |ev| {
                        set_email(event_target_value(&ev));
        }
                        id="registerEmail"
                        placeholder="Enter your email"
                    />
                </div>
                <div class="mb-3">
                    <label for="registerPassword" class="form-label">
                        {"Password"}
                    </label>
                    <input
                        type="password"
                        class="form-control"
                        prop:value=password
                        on:input= move |ev| {
                        set_password(event_target_value(&ev));
        }
                        id="registerPassword"
                        placeholder="Create a password"
                    />
                </div>
                <button type="submit" class="btn btn-success w-100">
                    {"Register"}
                </button>
            </form>
        </div>
    }
}