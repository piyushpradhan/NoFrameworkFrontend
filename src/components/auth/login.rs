use serde_json;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::rc::Rc;

use reqwest::header::{HeaderValue, CONTENT_TYPE};
use wasm_bindgen::prelude::*;
use web_sys::{window, Event, HtmlElement, HtmlInputElement, KeyboardEvent};

use crate::components::component::Component;
use crate::models::login_input::LoginInput;
use crate::routers::router::Router;
use crate::store;
use crate::utils::{api, Element};

use crate::models::user::User;

#[derive(Debug, serde::Deserialize)]
struct LoginResponse {
    id: i32,
    username: String,
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Copy, Clone)]
pub struct Login {}

impl Login {
    pub fn new() -> Self {
        let _ = store::get::<LoginInput>();
        Login {}
    }

    pub async fn handle_login(&self) {
        let router = store::get::<Router>();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let login_input = store::get::<LoginInput>();

        let data = format!(
            r#"{{ "username": "{}", "password": "{}" }}"#,
            login_input.username, login_input.password
        );

        let json: serde_json::Value = serde_json::from_str(&data).unwrap();

        let response = api::post("http://localhost:7878/auth/login", headers.clone(), json).await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let body = res.bytes().await.unwrap();
                    let body_str = String::from_utf8(body.to_vec()).unwrap();

                    let json_response: LoginResponse = serde_json::from_str(&body_str).unwrap();

                    store::set::<User>(User::new(
                        json_response.id,
                        json_response.username,
                        json_response.access_token,
                        json_response.refresh_token,
                    ));
                }
            }
            Err(err) => {
                println!("Error sending request: {}", err);
            }
        }

        router.render("/home");
    }
}

impl Component for Login {
    fn render(&self) -> HtmlElement {
        let mut login_component = Login::new();
        let document = window().unwrap().document().unwrap();
        let container = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        let mut heading = Element::new("h2");
        let mut login_form = Element::new("form");

        let mut username_row = Element::new("tr");
        let mut username_label = Element::new("td");
        let mut username_container = Element::new("td");
        let mut username_input = Element::new("input");

        let mut password_row = Element::new("tr");
        let mut password_label = Element::new("td");
        let mut password_container = Element::new("td");
        let mut password_input = Element::new("input");

        let mut submit_button_row = Element::new("tr");
        let mut submit_button_container = Element::new("td");
        let mut submit_button = Element::new("button");

        let mut form_table = Element::new("table");

        form_table.set(|table: &HtmlElement| {
            table.set_attribute("cellspacing", "10").unwrap();
        });

        heading.set(|h2: &HtmlElement| {
            h2.set_inner_text("Login");
        });

        username_label.set(|label: &HtmlElement| {
            label.set_attribute("align", "right").unwrap();
            label.set_inner_text("Username: ");
        });

        username_input
            .set(|input: &HtmlElement| {
                input.set_id("username");
                input.set_attribute("type", "text").unwrap();
                input.set_attribute("name", "username").unwrap();
                input.set_attribute("placeholder", "Username").unwrap();
            })
            .set(|input: &HtmlElement| {
                let handler = Closure::wrap(Box::new(|e: Event| {
                    let input = e
                        .current_target()
                        .unwrap()
                        .dyn_into::<HtmlInputElement>()
                        .unwrap();

                    let mut login_input = store::get::<LoginInput>();
                    login_input.set_username(input.value());
                }) as Box<dyn FnMut(_)>);

                input
                    .add_event_listener_with_callback("input", &handler.as_ref().unchecked_ref())
                    .unwrap();

                handler.forget();
            });

        password_label.set(|label: &HtmlElement| {
            label.set_attribute("align", "right").unwrap();
            label.set_inner_text("Password: ")
        });
        password_input
            .set(|input: &HtmlElement| {
                input.set_id("password");
                input.set_attribute("type", "password").unwrap();
                input.set_attribute("name", "password").unwrap();
                input.set_attribute("placeholder", "password").unwrap();
            })
            .set(|input: &HtmlElement| {
                let handler = Closure::wrap(Box::new(|e: Event| {
                    let input = e
                        .current_target()
                        .unwrap()
                        .dyn_into::<HtmlInputElement>()
                        .unwrap();

                    let mut login_input = store::get::<LoginInput>();
                    login_input.set_password(input.value());
                }) as Box<dyn FnMut(_)>);

                input
                    .add_event_listener_with_callback("input", &handler.as_ref().unchecked_ref())
                    .unwrap();

                handler.forget();
            });

        submit_button_container.set(|container: &HtmlElement| {
            container.set_attribute("colspan", "2").unwrap();
            container.set_attribute("align", "center").unwrap();
        });

        submit_button.set(|button: &HtmlElement| {
            button.set_inner_text("Submit");
            button.set_attribute("type", "submit").unwrap();
        });

        let built_login_form = login_form
            .set(|form: &HtmlElement| {
                form.set_attribute("action", "http://localhost:7878/auth/login")
                    .unwrap();
                form.set_attribute("method", "POST").unwrap();
            })
            .set(|form: &HtmlElement| {
                let handler = Closure::wrap(Box::new(move |event: Event| {
                    event.prevent_default();
                    let _ = wasm_bindgen_futures::future_to_promise(async move {
                        login_component.handle_login().await;
                        Ok::<JsValue, JsValue>(JsValue::undefined())
                    });
                }) as Box<dyn FnMut(_)>);

                let _ = form
                    .add_event_listener_with_callback("submit", handler.as_ref().unchecked_ref());

                handler.forget();
            })
            .child(
                form_table
                    .child(
                        username_row
                            .child(username_label.build())
                            .child(username_container.child(username_input.build()).build())
                            .build(),
                    )
                    .child(
                        password_row
                            .child(password_label.build())
                            .child(password_container.child(password_input.build()).build())
                            .build(),
                    )
                    .child(
                        submit_button_row
                            .child(submit_button_container.child(submit_button.build()).build())
                            .build(),
                    )
                    .build(),
            )
            .build();

        container.set_attribute("align", "center").unwrap();
        container.append_child(&heading.build()).unwrap();
        container.append_child(&built_login_form).unwrap();

        container
    }
}
