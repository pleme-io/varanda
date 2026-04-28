// varanda — family-facing PWA entry point.

use varanda::{
    api::accessible_services,
    hostname::{from_host, ViewMode},
    model::AccessibleService,
    session::read_session,
    view::Portal,
};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let mode = use_state(|| current_view_mode());
    let services = use_state(|| Option::<Result<Vec<AccessibleService>, String>>::None);
    let session = use_state(read_session);

    {
        let services = services.clone();
        let session_user = session.as_ref().map(|c| c.sub.clone());
        use_effect_with(session_user.clone(), move |user_opt| {
            if let Some(user) = user_opt.clone() {
                let services = services.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = accessible_services(&user).await;
                    services.set(Some(result));
                });
            }
            || ()
        });
    }

    let user_display = session
        .as_ref()
        .and_then(|c| c.email.clone().or_else(|| Some(c.sub.clone())))
        .unwrap_or_else(|| "guest".into());

    match (*services).as_ref() {
        None => html! {
            <main class="varanda-root">
                <h1>{ "loading…" }</h1>
                if session.is_none() {
                    <p>{ "Not signed in. " }<a href="/">{ "Sign in via passaporte" }</a></p>
                }
            </main>
        },
        Some(Ok(svcs)) => html! {
            <Portal mode={(*mode).clone()} user_display={user_display} services={svcs.clone()} />
        },
        Some(Err(e)) => html! {
            <main class="varanda-root">
                <h1>{ "couldn't reach crachá" }</h1>
                <pre>{ e }</pre>
            </main>
        },
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
