#[allow(clippy::wildcard_imports)]
use leptos::*;
use luhn::Luhn;

pub mod luhn;

fn main() {
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    let new_luhn = Luhn::new("");
    let (luhn, set_luhn) = create_signal(Luhn::new(""));
    let generate_valid_luhn_number = luhn().generate_valid_luhn_number();
    set_luhn(generate_valid_luhn_number);
    view! {
        <div style="flex ">
            <button on:click=move |_| {
                set_luhn(luhn().generate_valid_luhn_number());
            }>

                "Click me: " {move || luhn().to_string()}
            </button>
            // <progress
            //     class="p-16 w-full"
            //     max="50"
            //     // signals are functions, so `value=count` and `value=move || count.get()`
            //     // are interchangeable.
            //     value=count
            // />
        </div>
    }
}
