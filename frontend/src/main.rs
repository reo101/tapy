#[macro_use]
extern crate dotenvy_macro;

use dotenvy::dotenv;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/get/{id}")]
    Item { id: u32 },
    // #[at("/get?tags={}")]
    // Gallery { tags: String },
    #[at("/gallery")]
    Gallery,
    #[at("/add")]
    Add,
    #[not_found]
    #[at("/404")]
    NotFound,
}

mod components;

use components::add::Add;
use components::home::Home;
use components::item::Item;
use components::gallery::Gallery;
use components::not_found::NotFound;

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <
                Switch<Route>
                render={Switch::render(switch)}
            />
        </BrowserRouter>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::NotFound => html! {
            <NotFound />
        },
        Route::Item { id } => html! {
            <Item id={*id} />
        },
        // Route::Gallery { tags } => html! {
        //     <Gallery tags={tags.clone()} />
        // },
        Route::Gallery => html! {
            <Gallery tags={""} />
        },
        Route::Add => html! {
            <Add />
        },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dotenv().ok();

    yew::start_app::<Main>();
}
