use crate::error_template::{AppError, ErrorTemplate};
use gloo_timers::callback::Interval;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-weird-ssr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/subpage" view=SubPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <a href="/subpage">"Go Away"</a>
    }
}

#[component]
fn SubPage() -> impl IntoView {
    let res = create_resource(|| (), |_| do_stuff());

    view! {
        <Suspense
            fallback=|| view! {
                <Loading/>
            }
        >
            {move || res.get()}
            <h1>"Yay!"</h1>
        </Suspense>
    }
}

#[component]
pub(crate) fn Loading() -> impl IntoView {
    console_log("<Loading/>");

    let total_seconds = create_rw_signal(0);

    create_render_effect(move |_| {
        console_log("created effect!");
        Interval::new(1000, move || {
            total_seconds.update(|total| *total += 1);
        })
    });

    view! {
        <p>
            "Loading "
            {move || total_seconds.get()}
        </p>
    }
}

#[server]
async fn do_stuff() -> Result<(), ServerFnError> {
    use std::time::Duration;

    tokio::time::sleep(Duration::from_secs(10)).await;

    Ok(())
}
