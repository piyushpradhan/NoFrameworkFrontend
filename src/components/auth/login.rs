use reqwest::cookie::Jar;
use reqwest::{Client, Url};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;

use reqwest::header::{HeaderValue, CONTENT_TYPE};
use wasm_bindgen::prelude::*;
use web_sys::{window, Event, HtmlElement};

use crate::components::component::Component;
use crate::routers::router::Router;
use crate::store;
use crate::utils::storage::set_item;
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

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let data = r#"{
            "username": "latest_user",
            "password": "strong_password"
        }"#;

        let json: serde_json::Value = serde_json::from_str(&data).unwrap();

        let response = api::post("http://localhost:7878/auth/login", headers.clone(), json).await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let body = res.bytes().await.unwrap();
                    let body_str = String::from_utf8(body.to_vec()).unwrap();

                    let json_response: LoginResponse = serde_json::from_str(&body_str).unwrap();

                    let cookie = format!(
                        "token={};refresh={}",
                        json_response.access_token, json_response.refresh_token
                    );

                    // Set cookie to local storage
                    let jar = store::get::<Arc<Jar>>();
                    jar.add_cookie_str(&cookie, &"http://localhost:7878".parse::<Url>().unwrap());

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

        let _ = api::get("http://localhost:7878/test", headers.clone()).await;

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

        let mut username_label = Element::new("label");
        let mut username_input = Element::new("input");

        let mut password_label = Element::new("label");
        let mut password_input = Element::new("input");

        let mut submit_button = Element::new("button");

        heading.set(|h2: &HtmlElement| {
            h2.set_inner_text("Login");
        });

        username_label.set(|label: &HtmlElement| {
            label.set_attribute("for", "username").unwrap();
            label.set_inner_text("Username")
        });

        username_input.set(|input: &HtmlElement| {
            input.set_id("username");
            input.set_attribute("type", "text").unwrap();
            input.set_attribute("name", "username").unwrap();
            input.set_attribute("placeholder", "Username").unwrap();
        });

        password_label.set(|label: &HtmlElement| {
            label.set_attribute("for", "password").unwrap();
            label.set_inner_text("Password")
        });
        password_input.set(|input: &HtmlElement| {
            input.set_id("password");
            input.set_attribute("type", "password").unwrap();
            input.set_attribute("name", "password").unwrap();
            input.set_attribute("placeholder", "password").unwrap();
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
            .child(username_label.build())
            .child(username_input.build())
            .child(password_label.build())
            .child(password_input.build())
            .child(submit_button.build())
            .build();

        container.append_child(&heading.build()).unwrap();
        container.append_child(&built_login_form).unwrap();

        container
    }
}
