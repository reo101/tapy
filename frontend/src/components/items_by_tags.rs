use yew::functional::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub tags: String,
}

#[function_component(ItemsByTags)]
pub fn items_by_tags(props: &Props) -> Html {
    let tags = &props.tags;

    html! {
        <>
            <h1>{ "Items by Tags, Poggers" }</h1>
            <h2>{ tags }</h2>
        </>
    }
}
