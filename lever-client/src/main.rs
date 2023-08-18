use leptos::*;
use leptos_router::*;
use components::Header;

pub mod components;
pub mod api;
pub mod views;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Header />
            <main class="w-full max-w-3xl m-auto">
                <Routes>
                    <Route path="/" view=views::FeatureListView />
                    <Route path="/new" view=views::NewFeatureView />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App /> })
}
