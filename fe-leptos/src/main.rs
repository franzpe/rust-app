use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);

    view! {
        <div>
            <h1>{move || counter.get()}</h1>
            <button on:click= move |_| {
              set_counter.update(|count| *count += 1);
            }> "Increment me" </button>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App />});
}
