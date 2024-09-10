#[allow(clippy::wildcard_imports)]
use leptos::*;

mod luhn;

fn main() {
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div style="flex ">
            <button on:click=move |_| {
                set_count.update(|n| *n += 1);
            }>

                "Click me: " {move || count()}
            </button>
            <progress
                class="p-16 w-full"
                max="50"
                // signals are functions, so `value=count` and `value=move || count.get()`
                // are interchangeable.
                value=count
            />
        </div>
    }
}
