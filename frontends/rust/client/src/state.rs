use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub page: i32,
}

pub fn set_state(s: State) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let state_ele = document.get_element_by_id("state").unwrap();
    let state_raw = serde_json::to_string::<State>(&s).unwrap();
    state_ele.set_inner_html(state_raw.as_str());
}

pub fn get_state() -> State {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let state_ele = document.get_element_by_id("state").unwrap();
    let state_raw: String = state_ele.inner_html();
    let state = serde_json::from_str::<State>(state_raw.as_str()).unwrap();
    state
}
