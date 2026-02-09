use web_sys::DragEvent;
use yew::prelude::*;

use crate::components::block_canvas::{default_countdown_block, default_wait_block};
use crate::state::{AppAction, AppStateContext};

#[derive(Properties, PartialEq)]
pub struct DropZoneProps {
    pub index: usize,
}

#[function_component(BlockDropZone)]
pub fn block_drop_zone(props: &DropZoneProps) -> Html {
    let state = use_context::<AppStateContext>().expect("no context found");
    let active = use_state(|| false);
    let index = props.index;

    let ondragover = {
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
        })
    };

    let ondragenter = {
        let active = active.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            active.set(true);
        })
    };

    let ondragleave = {
        let active = active.clone();
        Callback::from(move |_: DragEvent| {
            active.set(false);
        })
    };

    let ondrop = {
        let active = active.clone();
        let state = state.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            active.set(false);
            if let Some(dt) = e.data_transfer() {
                if let Ok(data) = dt.get_data("text/plain") {
                    if let Some(from_str) = data.strip_prefix("existing:") {
                        if let Ok(from) = from_str.parse::<usize>() {
                            state.dispatch(AppAction::MoveBlock { from, to: index });
                        }
                    } else if let Some(block_type) = data.strip_prefix("new:") {
                        let block = match block_type {
                            "wait" => default_wait_block(),
                            "countdown" => default_countdown_block(),
                            _ => return,
                        };
                        state.dispatch(AppAction::AddBlock(block, index));
                    }
                }
            }
        })
    };

    let class = if *active {
        "drop-zone drop-zone-active"
    } else {
        "drop-zone"
    };

    html! {
        <div class={class} {ondragover} {ondragenter} {ondragleave} {ondrop} />
    }
}
