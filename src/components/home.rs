use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement};

use super::component::Component;

pub struct Home {}

impl Home {
    pub fn new() -> Home {
        Home {}
    }
}

impl Component for Home {
    fn render(&self) -> HtmlElement {
        let document = window().unwrap().document().unwrap();
        let container = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        let random_text = document
            .create_element("p")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        random_text.set_inner_html("Mounting components like a pro!");

        container.append_child(&random_text).unwrap();

        container
    }
}
