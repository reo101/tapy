use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();

    let go_add = Callback::from(move |_| history.push(crate::Route::Add));
    // let go_items = Callback::from(move |_| {
    //     history.push(crate::Route::ItemsByTags {
    //         tags: "".to_string(),
    //     })
    // });

    html! {
        <div>
            <h1>{ "Home pog" }</h1>
            <button onclick={go_add}>{ "Go Add" }</button>
        </div>
    }
    // <button onclick={go_items}>{ "Go Items" }</button>
}
