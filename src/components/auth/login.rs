use serde_json;
use std::collections::HashMap;
use tokio::runtime::Handle;
use tokio::sync::oneshot;

use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement, MouseEvent};

use crate::components::component::Component;
use crate::routers::router::Router;
use crate::store;
use crate::utils::Element;

use super::user::User;

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
    pub fn new() -> Login {
        Login {}
    }

    pub async fn handle_login(&mut self) {
        let router = store::get::<Router>();

        let mut payload = HashMap::new();
        payload.insert("username", "latest_user");
        payload.insert("password", "strong_password");

        let mut error = HashMap::new();
        error.insert(
            "message",
            "Something went wrong while parsing the user data",
        );

        let client = reqwest::blocking::Client::new();
        let res = client.post("http://localhost:7878/auth/login").form(&payload).header("Cookie", "token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IlwibGF0ZXN0X3VzZXJcIiIsInVpZCI6MTgsImV4cCI6MTcxMDM4MDUwOH0.4Vx_AP2YLkrkXq5IpByqzZC3eDJvK65dDhWdq-dhxf4;refresh=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IlwibGF0ZXN0X3VzZXJcIiIsInVpZCI6MTcsImV4cCI6MTcxMDU5NzY1OX0.1Knh2-U-QCzvElieLCeHfEW7wbkvSMY2IMmN-OgRJB8eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IlwibGF0ZXN0X3VzZXJcIiIsInVpZCI6MTgsImV4cCI6MTcxMDU5OTMwOH0.KwfERdQgu4PR4T0bDqJeNj9MPP0kP1Mzo7rFxPR2zkI").send();

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    let body = response.bytes().unwrap();
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

        let mut login_form = Element::new("form");
        let mut username_input = Element::new("input");
        let mut password_input = Element::new("input");
        let mut submit_button = Element::new("button");

        username_input.set(|input: &HtmlElement| {
            let _ = input.set_attribute("placeholder", "Username");
        });

        password_input.set(|input: &HtmlElement| {
            let _ = input.set_attribute("placeholder", "Password");
            let _ = input.set_attribute("type", "password");
        });

        let handle = Handle::current();
        submit_button
            .set(|button: &HtmlElement| {
                button.set_inner_text("Submit");
            })
            .set(|submit_button: &HtmlElement| {
                let handle_clone = handle.clone();
                let handler = Closure::wrap(Box::new(move |_event: MouseEvent| {
                    login_component.handle_login().await;
                }) as Box<dyn FnMut(_)>);

                let _ = submit_button
                    .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref());

                handler.forget();
            });

        let built_login_form = login_form
            .set(|form: &HtmlElement| {
                let handler =
                    Closure::wrap(Box::new(move |_event: MouseEvent| {}) as Box<dyn FnMut(_)>);

                let _ = form
                    .add_event_listener_with_callback("submit", handler.as_ref().unchecked_ref());

                handler.forget();
            })
            .child(username_input.build())
            .child(password_input.build())
            .child(submit_button.build())
            .build();

        container.append_child(&built_login_form).unwrap();

        container
    }
}
