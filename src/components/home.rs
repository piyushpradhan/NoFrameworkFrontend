use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement};

use crate::{store, utils::Element};

use super::component::Component;
use crate::models::user::User;

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

        let user_details = Element::new("p")
            .set(|p: &HtmlElement| {
                let user = store::get::<User>();
                p.set_inner_html(&user.username);
            })
            .build();

        random_text.set_inner_html("Mounting components like a pro!");

        container.append_child(&random_text).unwrap();
        container.append_child(&user_details).unwrap();

        container
    }
}
