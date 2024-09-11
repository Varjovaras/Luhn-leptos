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
    let (luhn, set_luhn) = create_signal(Luhn::new(""));
    let generate_valid_luhn_number = Luhn::generate_valid_luhn_number();
    set_luhn(generate_valid_luhn_number);
    view! {
        <div class="flex flex-row justify-center content-center py-4">
            <button
                class="border p-4"
                on:click=move |_| {
                    set_luhn(Luhn::generate_valid_luhn_number());
                }
            >

                "Click me: "
            </button>
            <div class="p-4">{move || luhn().to_string()}</div>

        </div>
    }
}
