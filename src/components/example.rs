use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use web_sys::{window, HtmlElement};

use super::component::Component;
use crate::models::user::User;
use crate::routers::router::Router;
use crate::store;
use crate::utils::Element;

// Example component implementation
pub struct ExampleComponent {
    text: String,
}

impl ExampleComponent {
    pub fn new(text: &str) -> ExampleComponent {
        ExampleComponent {
            text: text.to_string(),
        }
    }

    // Function to handle button click event
    fn handle_click(&mut self) {
        self.text = "Text Changed!".to_string();

        let router = store::get::<Router>();

        router.render("/login");
    }
}

impl Component for ExampleComponent {
    fn render(&self) -> HtmlElement {
        let mut component = ExampleComponent::new("This works!");
        let document = window().unwrap().document().unwrap();
        let container = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        let mut button = Element::new("button");
        let mut user_details = Element::new("p");
        let current_user = store::get::<User>();
        button.set(|btn: &HtmlElement| {
            btn.set_inner_html("This works!");
            let handler = Closure::wrap(Box::new(move |_event: MouseEvent| {
                component.handle_click();
            }) as Box<dyn FnMut(_)>);

            let _ = btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref());

            handler.forget();
        });
        user_details.set(|p: &HtmlElement| {
            p.set_inner_html(&current_user.username);
        });

        container.append_child(&button.build()).unwrap();
        container.append_child(&user_details.build()).unwrap();

        let p = document
            .create_element("p")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        p.set_inner_html(&self.text);
        container.append_child(&p).unwrap();

        container
    }
}
