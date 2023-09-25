use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(TestComponent)]
pub fn test_comp(props: &Props) -> Html {
    return html! {
        for props.children.iter()
    };
}

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();

        Callback::from(move |_| {
            counter.set(*counter.clone() + 1);
        })
    };

    html! {
        <main>
            <span>{"Counter: "} {*counter}</span>
            <button {onclick}>{ " My Btn "}</button>
        </main>
    }
}
