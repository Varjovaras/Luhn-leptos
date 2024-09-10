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
        <div style="display: flex">
            <button
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                }
                class:red=move || count() % 2 == 1
                style="position: absolute"
                style:left=move || format!("{}px", count() + 100)
                style:background-color=move || format!("rgb({}, {}, 100)", count(), 100)
            >

                "Click me: "
                {move || count()}
            </button>
            <progress
                class="p-8"
                max="50"
                // signals are functions, so `value=count` and `value=move || count.get()`
                // are interchangeable.
                value=count
            />
        </div>
    }
}
