// Auth flow — drives the sign-in screen + the OAuth redirect to
// passaporte (Authentik).
//
// Flow:
//   1. User loads www.quero.cloud — varanda calls cracha/me.
//   2a. cracha/me 200 → render the portal.
//   2b. cracha/me 401 (no/invalid cookie) → render the sign-in
//       screen with a single "Sign in" button.
//   3. Button → redirect to auth.quero.cloud's authorize endpoint.
//      Authentik renders its own login page (username + password,
//      and/or "Sign in with Google" depending on which sources are
//      enabled in the blueprint). Authentik issues the session
//      cookie on its callback domain.
//   4. Authentik redirects back to www.quero.cloud/. varanda
//       reloads, cracha/me now 200, portal renders.
//
// We don't handle the OAuth callback ourselves — Authentik is the
// OIDC provider AND owns the redirect URI. Pages can't run server
// code, but it doesn't need to: the cookie is already set by the
// time the browser comes back here.
//
// Source-agnostic: this code is identical whether Authentik
// authenticates the user via local username/password or Google
// federation. The cosmetic flag below toggles only the button copy
// + Google brand mark on the varanda landing screen.

use yew::prelude::*;

/// Authentik (passaporte) authorize endpoint. Configurable at build
/// time so dev environments can point at a local Authentik.
const DEFAULT_PASSAPORTE_URL: &str = "https://auth.quero.cloud";

/// OIDC client_id for varanda — must match the blueprint's
/// `varanda-oidc` provider.client_id.
const VARANDA_CLIENT_ID: &str = "varanda";

#[must_use]
pub fn passaporte_url() -> String {
    option_env!("PASSAPORTE_URL")
        .unwrap_or(DEFAULT_PASSAPORTE_URL)
        .to_string()
}

/// Build-time flag: render the Google brand mark + "Sign in with
/// Google" copy on the landing screen. Default OFF — varanda shows
/// a generic "Sign in" button that bounces to Authentik, which
/// itself decides whether to show a password form, a Google tile,
/// or both based on the blueprint's enabled sources.
///
/// Flip on: `VARANDA_GOOGLE_ENABLED=true trunk build --release` (or
/// pass the env var through the flake's buildPhase). Must be paired
/// with `enabled: true` on the Authentik blueprint's google-source
/// entry, otherwise the button copy lies about the actual flow.
#[must_use]
fn google_branding_enabled() -> bool {
    matches!(
        option_env!("VARANDA_GOOGLE_ENABLED"),
        Some("true" | "1" | "yes")
    )
}

/// Build the authorize URL. Authentik's path is
/// `/application/o/authorize/?client_id=…` — same as any OIDC IdP.
/// The `redirect_uri` MUST appear (regex-matched) in the
/// blueprint's redirect_uris list.
#[must_use]
pub fn authorize_url(redirect_back_to: &str) -> String {
    let nonce = generate_nonce();
    format!(
        "{base}/application/o/authorize/?client_id={cid}&response_type=code&scope=openid+email+profile&redirect_uri={ru}&state={nonce}",
        base = passaporte_url(),
        cid = VARANDA_CLIENT_ID,
        ru = url_encode(redirect_back_to),
        nonce = nonce,
    )
}

#[derive(Properties, PartialEq)]
pub struct SignInScreenProps {
    /// Optional message to surface above the button (e.g.
    /// "session expired"). None renders the default copy.
    #[prop_or_default]
    pub notice: Option<String>,
}

/// The unauthenticated landing screen. Rendered when cracha/me
/// returns 401.
#[function_component(SignInScreen)]
pub fn sign_in_screen(props: &SignInScreenProps) -> Html {
    let on_click = Callback::from(move |_| {
        let here = current_url();
        let url = authorize_url(&here);
        let _ = web_sys::window().and_then(|w| w.location().set_href(&url).ok());
    });

    let google_branded = google_branding_enabled();

    html! {
        <main class="varanda-root varanda-signin">
            <header class="varanda-header">
                <span class="varanda-mark" aria-hidden="true">
                    <svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                        <path d="M8 56 C 8 32, 24 8, 32 8 S 56 32, 56 56" />
                    </svg>
                </span>
                <h1>{ "Saguão" }</h1>
            </header>
            <section class="varanda-signin-card">
                if let Some(notice) = &props.notice {
                    <p class="varanda-notice">{ notice }</p>
                }
                <p class="varanda-signin-blurb">
                    { "Sign in to access your services in the saguão fleet." }
                </p>
                if google_branded {
                    <button class="varanda-signin-button" onclick={on_click}>
                        <span class="varanda-google-mark" aria-hidden="true">
                            <svg viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg">
                                <path d="M24 9.5c3.54 0 6.71 1.22 9.21 3.6l6.85-6.85C35.9 2.38 30.47 0 24 0 14.62 0 6.51 5.38 2.56 13.22l7.98 6.19C12.43 13.72 17.74 9.5 24 9.5z" fill="#EA4335"/>
                                <path d="M46.98 24.55c0-1.57-.15-3.09-.38-4.55H24v9.02h12.94c-.58 2.96-2.26 5.48-4.78 7.18l7.73 6c4.51-4.18 7.09-10.36 7.09-17.65z" fill="#4285F4"/>
                                <path d="M10.53 28.59c-.48-1.45-.76-2.99-.76-4.59s.27-3.14.76-4.59l-7.98-6.19C.92 16.46 0 20.12 0 24c0 3.88.92 7.54 2.56 10.78l7.97-6.19z" fill="#FBBC05"/>
                                <path d="M24 48c6.48 0 11.93-2.13 15.89-5.81l-7.73-6c-2.15 1.45-4.92 2.3-8.16 2.3-6.26 0-11.57-4.22-13.47-9.91l-7.98 6.19C6.51 42.62 14.62 48 24 48z" fill="#34A853"/>
                            </svg>
                        </span>
                        <span>{ "Sign in with Google" }</span>
                    </button>
                    <p class="varanda-signin-fineprint">
                        { "Authentication via Google. You'll be redirected to " }
                        <code>{ "auth.quero.cloud" }</code>
                        { ", sign in there once, and return here automatically." }
                    </p>
                } else {
                    <button class="varanda-signin-button varanda-signin-button-plain" onclick={on_click}>
                        <span>{ "Sign in to Saguão" }</span>
                    </button>
                    <p class="varanda-signin-fineprint">
                        { "You'll be redirected to " }
                        <code>{ "auth.quero.cloud" }</code>
                        { " to enter your username and password, then returned here automatically." }
                    </p>
                }
            </section>
        </main>
    }
}

fn current_url() -> String {
    web_sys::window()
        .and_then(|w| w.location().href().ok())
        .unwrap_or_else(|| "https://www.quero.cloud/".into())
}

fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}

/// Lightweight nonce — Authentik validates state mostly to prevent
/// CSRF on the callback; varanda doesn't read state on return
/// (the cookie is what matters).
fn generate_nonce() -> String {
    use std::time::SystemTime;
    let n = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |d| d.as_micros());
    format!("{n:x}")
}
