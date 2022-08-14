use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/get/{id}")]
    Item { id: u32 },
    #[at("/get/?tags={}")]
    ItemsByTags { tags: String },
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
use components::items_by_tags::ItemsByTags;
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
            <Item id={id.clone()} />
        },
        Route::ItemsByTags { tags } => html! {
            <ItemsByTags tags={tags.clone()} />
        },
        Route::Add => html! {
            <Add />
        },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}
