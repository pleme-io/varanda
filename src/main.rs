// varanda — family-facing PWA entry point.
//
// State machine:
//   Loading   → render spinner, fetch /me
//   SignedOut → render SignInScreen (cracha returned 401 / network)
//   SignedIn  → render Portal (+ AdminPanel when role=admin)
//
// The session cookie (X-Saguao-Session) is set by passaporte
// (Authentik) on the *.quero.cloud domain. varanda doesn't read or
// validate it — the cookie travels via `credentials: include` on
// every cracha call, cracha JWT-validates server-side. This keeps
// the WASM bundle free of crypto deps + means session lifetime is
// fully controlled by passaporte.

use varanda::{
    admin::AdminPanel,
    api::{fetch_me, ApiError},
    auth::SignInScreen,
    hostname::{from_host, ViewMode},
    model::{MeResponse, Role},
    view::Portal,
};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum AuthState {
    Loading,
    SignedOut(Option<String>),
    SignedIn(MeResponse),
}

#[function_component(App)]
fn app() -> Html {
    let mode = use_state(current_view_mode);
    let auth = use_state(|| AuthState::Loading);

    {
        let auth = auth.clone();
        use_effect_with((), move |_| {
            let auth = auth.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_me().await {
                    Ok(me) => auth.set(AuthState::SignedIn(me)),
                    Err(ApiError::Unauthorized) => auth.set(AuthState::SignedOut(None)),
                    Err(ApiError::Forbidden) => {
                        auth.set(AuthState::SignedOut(Some("forbidden".into())));
                    }
                    Err(ApiError::Other(msg)) => {
                        auth.set(AuthState::SignedOut(Some(format!("crachá: {msg}"))));
                    }
                }
            });
            || ()
        });
    }

    match (*auth).clone() {
        AuthState::Loading => html! {
            <main class="varanda-root varanda-loading">
                <p>{ "loading…" }</p>
            </main>
        },
        AuthState::SignedOut(notice) => html! {
            <SignInScreen notice={notice} />
        },
        AuthState::SignedIn(me) => {
            let user_display = if me.display_name.is_empty() {
                me.email.clone()
            } else {
                me.display_name.clone()
            };
            html! {
                <>
                    <Portal
                        mode={(*mode).clone()}
                        user_display={user_display}
                        services={me.services.clone()}
                    />
                    if me.role == Role::Admin {
                        <AdminPanel />
                    }
                </>
            }
        }
    }
}

fn current_view_mode() -> ViewMode {
    let host = web_sys::window()
        .and_then(|w| w.location().host().ok())
        .unwrap_or_else(|| "quero.cloud".into());
    from_host(&host)
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
