use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Home {
    let history = use_history().unwrap();

    let onclick_callback = Callback::from(move |_| history.push(crate::Route::Home));

    html! {
        <div>
            <h1>{ "Not Found" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
        </div>
    }
}
