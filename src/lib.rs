mod components;
mod models;
mod routers;
mod store;
mod utils;

use std::sync::Arc;

use components::{auth::login::Login, example::ExampleComponent, home::Home};
use reqwest::{cookie::Jar, Client, ClientBuilder, Url};
use routers::router::Router;
use utils::storage::get_item;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Initialize page router
    let mut router = store::get::<Router>();
    // Initialize request client
    let cookie_jar = Arc::new(Jar::default());
    let cookie = get_item("cookie");
    cookie_jar.add_cookie_str(&cookie, &"http://localhost:7878".parse::<Url>().unwrap());
    let client_builder: ClientBuilder = reqwest::Client::builder();
    let client = client_builder
        .cookie_provider(Arc::clone(&cookie_jar))
        .build()
        .unwrap();

    store::set::<Arc<Jar>>(cookie_jar);

    let _global_client = store::set::<Client>(client);

    // Add routes and components
    router.add_route("/", Box::new(ExampleComponent::new("Not the home page")));
    router.add_route("/about", Box::new(ExampleComponent::new("About Page")));
    router.add_route("/contact", Box::new(ExampleComponent::new("Contact Page")));
    router.add_route("/home", Box::new(Home::new()));
    router.add_route("/login", Box::new(Login::new()));

    // Get current path and render corresponding component
    let location = web_sys::window().unwrap().location();
    let pathname = location.pathname().unwrap();
    let path = pathname.to_string();
    router.render(&path);

    Ok(())
}
