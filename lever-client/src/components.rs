use leptos::*;
use lever_core::Feature;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {

    view! { cx,
        <header class="w-full max-w-3xl m-auto">
            <nav>
                <h1 class="text-8xl bg-pink-400 text-yellow-200 p-4">lever</h1>
            </nav>
        </header>
    }
}

#[component]
pub fn FeatureList(cx: Scope, features: Vec<Feature>) -> impl IntoView {
    view! { cx,
        <ul>
            {features
                .into_iter()
                .map(|f| view! { cx,
                    <FeatureListItem feature={f} />
                })
                .collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
fn FeatureListItem(cx: Scope, feature: Feature) -> impl IntoView {
    view! { cx,
        <li>
            {feature.name}
            <input class="cursor-not-allowed" type="checkbox" checked={feature.enabled} />
        </li>
    }
}