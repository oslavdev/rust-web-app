use yew::prelude::*;

struct Model {
    value: i64
}

#[function_component(Home)]
pub fn home_page() -> Html {
    let state = use_state(|| Model {
        value: 0
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
    };

    html! {
        <div class={classes!("container", "mx-auto", "my-10")} >
            <h1> {"Home page"} </h1>
            <p> {"Clicked: "} { state.value }</p>
            <button class="mt-5 rounded bg-gray-400 p-2" {onclick}>
                { "Click me" }
            </button>
        </div>
    }
}
