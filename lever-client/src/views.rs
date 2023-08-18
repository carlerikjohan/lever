use leptos::*;
use crate::api::{load_features, create_feature};
use crate::components::FeatureList;

#[component]
pub fn FeatureListView(cx: Scope) -> impl IntoView {
    let load_features = create_resource(
        cx,
        || (),
        |_| async move {
            let features = load_features().await;
            match features {
                Some(features) => features,
                None => vec![],
            }
        }
    );

    view! { cx,
        <div class="p-4 border border-b-0 border-solid border-gray-400 flex justify-end cursor-context-menu">
            <a href="/new" class="h-[38px] p-2 border border-solid border-green-700 bg-green-600 text-white hover:bg-green-400 hover:border-green-500">"New"</a>
        </div>
        <div class="relative border border-solid border-gray-400 p-4">
            
            {move || load_features.read(cx).map(|features| view! { cx, <FeatureList features />}) }
        </div>
    }
}

#[component]
pub fn NewFeatureView(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "".to_string());
    let (enabled, set_enabled) = create_signal(cx, false);

    let create_feature_action = create_action(cx, |input: &(String, bool)| {
        let input = input.to_owned();

        async move {
            create_feature(input.0, input.1).await
        }
    });

    view! { cx,
        <div class="border border-solid border-gray-400">
            <div class="p-4 border-b border-solid border-gray-400">
                <a href="/" class="text-blue-500 hover:uppercase">"<- Go back"</a>
            </div>
            <form
                class="flex flex-col gap-4 p-4"
                on:submit=move |ev| {
                    ev.prevent_default();
                    create_feature_action.dispatch((name.get(), enabled.get()));
                }
            >
                <label class="flex flex-col">
                    "Name"
                    <input
                        class="border border-solid border-gray-400 rounded-sm cursor-cell"
                        type="text"
                        on:input=move |ev| {
                            set_name.set(event_target_value(&ev))
                        }
                        prop:value=name.get()
                    />
                </label>
                <label>
                        "Enabled "
                        <input
                            type="checkbox"
                            class="cursor-text"
                            on:change=move |ev| {
                                set_enabled.set(event_target_checked(&ev));
                            }
                            prop:checked=enabled.get()
                        />
                </label>
                <input type="submit" class="p-2 border border-solid border-green-700 bg-green-600 text-white hover:bg-green-400 hover:border-green-500 cursor-crosshair" />
            </form>
        </div>
    }
}