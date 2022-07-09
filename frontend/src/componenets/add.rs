use std::{
    fmt::{Debug, Display},
    rc::Rc,
};
use web_sys::HtmlInputElement;
use yew::{
    events::Event, function_component, functional::*, html, use_node_ref, use_reducer, Callback,
    Reducible, TargetCast,
};

pub enum Action {
    AddTag(String),
    // RemoveTag(String),
    ClearTags,
}

#[derive(Clone)]
struct Tags {
    tags: Vec<String>,
}

impl Default for Tags {
    fn default() -> Self {
        Self {
            tags: vec!["Alo".to_string(), "da".to_string()],
        }
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tags.fmt(f)
    }
}

impl Reducible for Tags {
    type Action = Action;

    fn reduce(mut self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let state = Rc::make_mut(&mut self);

        match action {
            Action::AddTag(tag) => {
                // state.push(tag);
                println!("Add tag {}", tag);
                Self {
                    tags: {
                        let mut new_tags = self.tags.clone();
                        new_tags.push(tag);
                        new_tags
                    },
                }
                .into()
            }
            // Action::RemoveTag(tag) => {
            //     // state.retain(||)
            //     println!("Remove tag {}", tag);
            // }
            Action::ClearTags => Self { tags: vec![] }.into(),
        }
        // self
    }
}

#[function_component(Add)]
pub fn add() -> Html {
    let tags = use_reducer(Tags::default);

    let input_ref = use_node_ref();

    // let on_change = {
    //     let input = input_ref.cast::<HtmlInputElement>();
    //
    //     Callback::from(move |_| {
    //         if let Some(input) = input.clone() {
    //             input_value_handle.set(input.value());
    //         }
    //     })
    //
    //     // if let Some(input) = input {
    //     //     input_value_handle.set(input.value());
    //     // }
    // };

    let add_tag = {
        let tags = tags.dispatcher();
        let input_ref = input_ref.clone();
        let input = input_ref.cast::<HtmlInputElement>();

        Callback::from(move |_| {
            gloo::console::log!(format!("input_ref: {:?}", &input_ref));
            gloo::console::log!(format!(
                "input_ref_value: {:?}",
                input_ref.get().unwrap().to_string()
            ));
            gloo::console::log!(format!("input: {:?}", &input));
            // tags.dispatch(Action::AddTag(input.clone().unwrap().value()))
            tags.dispatch(Action::AddTag("Alo da".to_string()))
        })
    };

    let clear_tags = {
        let tags = tags.dispatcher();
        // let input = input_ref.cast::<HtmlInputElement>();
        Callback::from(move |_| tags.dispatch(Action::ClearTags))
    };

    // let remove_tag = {
    //     let tags = tags.dispatcher();
    //     let input = input_ref.cast::<HtmlInputElement>();
    //     Callback::from(move |_| tags.dispatch(Action::RemoveTag(input.clone().unwrap().value())))
    // };

    // onchange={on_change}
    html! {
        <>
            <h1>{ "Add here" }</h1>
            <form>
                <span>{ tags.tags.clone() }</span>
                <input ref={input_ref} type="text"/>
                <button onclick={add_tag}> { "Add tag" } </button>
                <button onclick={clear_tags}> { "Clear tags" } </button>
                <input type="file"/>
            </form>
        </>
    }
}
