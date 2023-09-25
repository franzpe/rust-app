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
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h2 class="text-red-500">{ "HMR PLEASE !!!" }</h2>
            <h1>{ "Hello World!" }</h1>
            <span>{"Counter: "} {*counter}</span>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <TestComponent>
                <h1>{ "Hello World!" }</h1>
                <h1>{ "Hello World!" }</h1>
            </TestComponent>
            <button {onclick}>{ " My Btn "}</button>
        </main>
    }
}
