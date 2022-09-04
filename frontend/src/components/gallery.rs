use reqwasm::http::Request;
use yew::functional::*;
use yew::prelude::*;

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

                let fetched_items: Vec<i32> =
                    Request::get(&format!("{}/get/all", dotenv!("BACKEND_URL")).to_owned())
                        .header("tags", tags.as_str())
                        .header("Content-Type", "application/json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                items.set(
                    fetched_items
                        .into_iter()
                        .map(|item_id| Item {
                            id: item_id,
                            tags: vec!["mqu".to_string()],
                        })
                        .collect(),
                );
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

#[derive(Clone, PartialEq, Eq)]
pub struct Item {
    id: i32,
    tags: Vec<String>,
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
            let id = item.id;
            let tags = Some(item.tags.iter().fold(String::new(), |acc, tag| {
                [acc, tag.to_owned(), ", ".to_string()].join("")
            }));

            html! {
                <div>
                    // <@{ "iframe" }
                    //     src = { format!("{}/get/{}", dotenv!("BACKEND_URL"), item) }
                    //     sandbox = "allow-downloads-without-user-activation"
                    // >
                    <@{ "object" }
                        data = { format!("{}/get/{}", dotenv!("BACKEND_URL"), id) }
                    >
                    </@>
                    if let Some(tags) = tags {
                    <span>
                        { tags }
                    </span>
                }
                </div>
            }
        })
        .collect::<Html>()
}
