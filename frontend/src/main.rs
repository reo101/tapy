use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            {"Hello!"}
        </div>
    }

    // <BrowserRouter>
    //     <
    //         Switch<Route>
    //         render={Switch::render(switch)}
    //     />
    // </BrowserRouter>
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
