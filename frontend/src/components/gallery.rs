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

                let fetched_items: Vec<Item> =
                    Request::get(&format!("{}/get/all", dotenv!("BACKEND_URL")).to_owned())
                        .header("tags", tags.as_str())
                        .header("Content-Type", "application/json")
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
    fn is_something(exts: Vec<&'static str>) -> impl Fn(&Item) -> bool {
        move |item: &Item| -> bool { exts.iter().any(|ext| item.path.ends_with(ext)) }
    }

    let is_video = is_something(vec!["mp4, webm"]);
    let is_picture = is_something(vec!["png", "gif", "jpg", "jpeg", "svg"]);

    items
        .iter()
        .map(|item| {
            html! {
                <div>
                <@{
                    if is_video(item) {
                        "video"
                    } else if is_picture(item) {
                        "img"
                    } else {
                        "iframe"
                    }}
                src = { format!("{}/get/{}", dotenv!("BACKEND_URL"), item.id) }
                    >
                    </@>
                    <span hidden=true>
                    {
                        // TODO: change structure to allow sending tags together with item
                        "TODO"
                    }
                    </span>
                </div>
            }
        })
        .collect::<Html>()
}
