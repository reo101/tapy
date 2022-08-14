// use common::models::item::Item;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::functional::*;
use yew::prelude::*;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Item {
    pub id: i32,
    pub path: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct GalleryProps {
    // pub tags: Option<String>,
    pub tags: String,
}

#[function_component(Gallery)]
pub fn gallery(gallery_props: &GalleryProps) -> Html {
    let tags = gallery_props.clone().tags;

    let items = use_state(Vec::new);
    {
        let items = items.clone();
        use_effect(move || {
            let items = items.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if items.len() != 0 {
                    return;
                }

                let fetched_items: Vec<Item> = Request::get("http://localhost:8080/api/get")
                    .header("tags", tags.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                items.set(fetched_items);
            });

            || ()
        });
    }

    html! {
        <>
            <h1>{ "Items by tags:" }</h1>
            <ItemList items={ (*items).clone() } />
        </>
    }
}

#[derive(Clone, Properties, PartialEq, Eq)]
pub struct ItemListProps {
    items: Vec<Item>,
}

#[function_component(ItemList)]
pub fn item_list(ItemListProps { items }: &ItemListProps) -> Html {
    items
        .iter()
        .map(|item| {
            html! {
                <p>
                    <@{if item.path.ends_with("mp4") { "video" } else { "img" }}
                        src = { format!("http://localhost:8080/api/get/{}", item.id) }
                     >
                    </@>
                </p>
            }
        })
        .collect::<Html>()
}
