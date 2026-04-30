// Admin panel — shown when /me.role == admin. Three sections:
//   1. Recent audit events (read-only tail)
//   2. Add grant form (user_id + service + verb + optional note)
//   3. Revoke grant form (same triple)
//
// Both forms POST/DELETE crachá's /admin/grants endpoint with the
// session cookie. Errors surface inline. Successful mutations
// trigger a re-fetch of the audit tail.

use crate::api::{add_grant, fetch_audit, revoke_grant};
use crate::model::{AddGrantRequest, AuditEvent, RevokeGrantRequest};
use yew::prelude::*;

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    let audit = use_state(|| Option::<Result<Vec<AuditEvent>, String>>::None);
    let refresh_tick = use_state(|| 0u32);

    {
        let audit = audit.clone();
        let tick = *refresh_tick;
        use_effect_with(tick, move |_| {
            let audit = audit.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = fetch_audit(50)
                    .await
                    .map(|r| r.events)
                    .map_err(|e| e.message());
                audit.set(Some(res));
            });
            || ()
        });
    }

    let on_mutation = {
        let refresh_tick = refresh_tick.clone();
        Callback::from(move |_| {
            refresh_tick.set(*refresh_tick + 1);
        })
    };

    html! {
        <section class="varanda-admin">
            <h2>{ "Admin" }</h2>
            <div class="varanda-admin-grid">
                <GrantForm on_committed={on_mutation.clone()} />
                <RevokeForm on_committed={on_mutation.clone()} />
                <AuditTail events={(*audit).clone()} />
            </div>
        </section>
    }
}

// ── Grant form ─────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct GrantFormProps {
    on_committed: Callback<()>,
}

#[function_component(GrantForm)]
fn grant_form(props: &GrantFormProps) -> Html {
    let user_id = use_state(String::new);
    let service = use_state(String::new);
    let verb = use_state(|| "read".to_string());
    let note = use_state(String::new);
    let status = use_state(|| Option::<Result<String, String>>::None);
    let busy = use_state(|| false);

    let submit = {
        let user_id = user_id.clone();
        let service = service.clone();
        let verb = verb.clone();
        let note = note.clone();
        let status = status.clone();
        let busy = busy.clone();
        let on_committed = props.on_committed.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if *busy {
                return;
            }
            busy.set(true);
            let req = AddGrantRequest {
                user_id: (*user_id).clone(),
                service: (*service).clone(),
                verb: (*verb).clone(),
                expires_at: None,
                note: if note.is_empty() { None } else { Some((*note).clone()) },
            };
            let status = status.clone();
            let busy = busy.clone();
            let on_committed = on_committed.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match add_grant(&req).await {
                    Ok(()) => {
                        status.set(Some(Ok("granted".into())));
                        on_committed.emit(());
                    }
                    Err(e) => status.set(Some(Err(e.message()))),
                }
                busy.set(false);
            });
        })
    };

    html! {
        <form class="varanda-admin-form" onsubmit={submit}>
            <h3>{ "Add grant" }</h3>
            <Field label="User ID (uuid)" value={(*user_id).clone()} on_input={set_string(&user_id)} placeholder="00000000-0000-…" />
            <Field label="Service slug" value={(*service).clone()} on_input={set_string(&service)} placeholder="drive" />
            <VerbSelect value={(*verb).clone()} on_input={set_string(&verb)} />
            <Field label="Note (optional)" value={(*note).clone()} on_input={set_string(&note)} placeholder="why this grant?" />
            <button type="submit" disabled={*busy} class="varanda-admin-submit">
                if *busy { { "granting…" } } else { { "Grant access" } }
            </button>
            { render_status(&*status) }
        </form>
    }
}

// ── Revoke form ───────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct RevokeFormProps {
    on_committed: Callback<()>,
}

#[function_component(RevokeForm)]
fn revoke_form(props: &RevokeFormProps) -> Html {
    let user_id = use_state(String::new);
    let service = use_state(String::new);
    let verb = use_state(|| "read".to_string());
    let status = use_state(|| Option::<Result<String, String>>::None);
    let busy = use_state(|| false);

    let submit = {
        let user_id = user_id.clone();
        let service = service.clone();
        let verb = verb.clone();
        let status = status.clone();
        let busy = busy.clone();
        let on_committed = props.on_committed.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if *busy {
                return;
            }
            busy.set(true);
            let req = RevokeGrantRequest {
                user_id: (*user_id).clone(),
                service: (*service).clone(),
                verb: (*verb).clone(),
            };
            let status = status.clone();
            let busy = busy.clone();
            let on_committed = on_committed.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match revoke_grant(&req).await {
                    Ok(()) => {
                        status.set(Some(Ok("revoked".into())));
                        on_committed.emit(());
                    }
                    Err(e) => status.set(Some(Err(e.message()))),
                }
                busy.set(false);
            });
        })
    };

    html! {
        <form class="varanda-admin-form" onsubmit={submit}>
            <h3>{ "Revoke grant" }</h3>
            <Field label="User ID" value={(*user_id).clone()} on_input={set_string(&user_id)} placeholder="00000000-0000-…" />
            <Field label="Service slug" value={(*service).clone()} on_input={set_string(&service)} placeholder="drive" />
            <VerbSelect value={(*verb).clone()} on_input={set_string(&verb)} />
            <button type="submit" disabled={*busy} class="varanda-admin-submit varanda-danger">
                if *busy { { "revoking…" } } else { { "Revoke access" } }
            </button>
            { render_status(&*status) }
        </form>
    }
}

// ── Audit tail ─────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct AuditTailProps {
    events: Option<Result<Vec<AuditEvent>, String>>,
}

#[function_component(AuditTail)]
fn audit_tail(props: &AuditTailProps) -> Html {
    html! {
        <section class="varanda-admin-audit">
            <h3>{ "Recent activity" }</h3>
            {
                match &props.events {
                    None => html! { <p class="varanda-muted">{ "loading audit log…" }</p> },
                    Some(Err(e)) => html! { <p class="varanda-error">{ e }</p> },
                    Some(Ok(events)) if events.is_empty() => html! {
                        <p class="varanda-muted">{ "no events yet" }</p>
                    },
                    Some(Ok(events)) => html! {
                        <ul class="varanda-audit-list">
                            { for events.iter().map(audit_row) }
                        </ul>
                    },
                }
            }
        </section>
    }
}

fn audit_row(e: &AuditEvent) -> Html {
    html! {
        <li class="varanda-audit-event">
            <span class="varanda-audit-ts">{ format_ts(&e.ts) }</span>
            <span class="varanda-audit-action">{ &e.action }</span>
            <span class="varanda-audit-actor">{ &e.actor_email }</span>
            <span class="varanda-audit-target">{ &e.target_id }</span>
        </li>
    }
}

/// Trim the timestamp to HH:MM date — readable at a glance.
fn format_ts(ts: &str) -> String {
    // ISO 8601 prefix: YYYY-MM-DDTHH:MM. Take the first 16 chars, strip the T.
    ts.chars()
        .take(16)
        .collect::<String>()
        .replacen('T', " ", 1)
}

// ── Shared input bits ──────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct FieldProps {
    label: String,
    value: String,
    on_input: Callback<String>,
    #[prop_or_default]
    placeholder: String,
}

#[function_component(Field)]
fn field(props: &FieldProps) -> Html {
    let cb = props.on_input.clone();
    let oninput = Callback::from(move |e: InputEvent| {
        let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
        cb.emit(target.value());
    });
    html! {
        <label class="varanda-field">
            <span class="varanda-field-label">{ &props.label }</span>
            <input
                type="text"
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                {oninput}
            />
        </label>
    }
}

#[derive(Properties, PartialEq)]
struct VerbSelectProps {
    value: String,
    on_input: Callback<String>,
}

#[function_component(VerbSelect)]
fn verb_select(props: &VerbSelectProps) -> Html {
    let cb = props.on_input.clone();
    let onchange = Callback::from(move |e: Event| {
        let target = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
        cb.emit(target.value());
    });
    html! {
        <label class="varanda-field">
            <span class="varanda-field-label">{ "Verb" }</span>
            <select value={props.value.clone()} {onchange}>
                <option value="read" selected={props.value == "read"}>{ "read" }</option>
                <option value="write" selected={props.value == "write"}>{ "write" }</option>
                <option value="admin" selected={props.value == "admin"}>{ "admin" }</option>
                <option value="*" selected={props.value == "*"}>{ "all (*)" }</option>
            </select>
        </label>
    }
}

fn set_string(state: &UseStateHandle<String>) -> Callback<String> {
    let state = state.clone();
    Callback::from(move |v: String| state.set(v))
}

fn render_status(status: &Option<Result<String, String>>) -> Html {
    match status {
        None => html! {},
        Some(Ok(msg)) => html! { <p class="varanda-success">{ "✓ " }{ msg }</p> },
        Some(Err(e)) => html! { <p class="varanda-error">{ "✗ " }{ e }</p> },
    }
}
