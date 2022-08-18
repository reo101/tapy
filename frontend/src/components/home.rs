use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();

    let go_see = Callback::from(move |_| history.push(crate::Route::Gallery));

    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button onclick={go_see}>{ "Go See" }</button>
        </div>
    }
}
