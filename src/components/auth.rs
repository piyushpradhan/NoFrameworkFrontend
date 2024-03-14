use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement};

use super::component::Component;

pub struct AuthComponent {}

impl AuthComponent {
    pub fn new() -> AuthComponent {
        AuthComponent {}
    }
}

impl Component for AuthComponent {
    fn render(&self) -> HtmlElement {
        let document = window().unwrap().document().unwrap();
        let container = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        container
    }
}
