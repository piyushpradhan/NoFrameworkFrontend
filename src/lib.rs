mod components;
mod routers;
mod store;
mod utils;

use components::{auth::AuthComponent, example::ExampleComponent, home::Home};
use routers::router::Router;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let mut router = store::get::<Router>();

    // Add routes and components
    router.add_route("/", Box::new(ExampleComponent::new("Not the home page")));
    router.add_route("/about", Box::new(ExampleComponent::new("About Page")));
    router.add_route("/contact", Box::new(ExampleComponent::new("Contact Page")));
    router.add_route("/home", Box::new(Home::new()));
    router.add_route("/auth", Box::new(AuthComponent::new()));

    // Get current path and render corresponding component
    let location = web_sys::window().unwrap().location();
    let pathname = location.pathname().unwrap();
    let path = pathname.to_string();
    router.render(&path);

    Ok(())
}
