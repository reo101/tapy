use yew::functional::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: u32,
}

#[function_component(Item)]
pub fn item(props: &Props) -> Html {
    let id = props.id;

    html! {
        <h1>{ "Home Poggers" }</h1>
    }
}
