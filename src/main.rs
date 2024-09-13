#[allow(clippy::wildcard_imports)]
use leptos::*;
use luhn::Luhn;

pub mod luhn;

fn main() {
    mount_to_body(|| view! { <Layout /> });
}

#[component]
fn Layout() -> impl IntoView {
    view! {
        <div class="flex flex-col h-screen w-screen justify-center  content-center">
            <App />
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (luhn, set_luhn) = create_signal(Luhn::default());
    let (length, set_length) = create_signal(luhn().get_length());

    view! {
        <div class="flex flex-col justify-center content-center py-4">
            <button
                class="border p-4"
                on:click=move |_| {
                    set_luhn(Luhn::new_with_length(length()));
                }
            >

                "Click me: "
            </button>
            <div class="p-4">{move || luhn().to_string()}</div>
            <button
                class="border p-4"
                on:click=move |_| {
                    set_length.update(|n| *n += 1000);
                    set_luhn(Luhn::new_with_length(length()));
                }
            >

                "Increment length: "
            </button>
            <div class="p-4">{move || length().to_string()}</div>

        </div>
    }
}
