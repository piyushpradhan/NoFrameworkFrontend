use web_sys::window;

use crate::components::component::Component;

// Router for managing routes and rendering components
pub struct Router {
    routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self {
        Router { routes: Vec::new() }
    }
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn add_route(&mut self, path: &str, component: Box<dyn Component + Send + Sync>) {
        self.routes.push(Route {
            path: path.to_string(),
            component,
        });
    }

    pub fn render(&self, path: &str) {
        let route = self.routes.iter().find(|r| r.path == path);

        println!(
            "routes: {:#?} path: {:?} single_route: {:?}",
            &self.routes.iter().for_each(|r| {
                println!("nested path: {}", r.path);
            }),
            &path,
            &route.unwrap().path,
        );

        if let Some(route) = route {
            let root = window().unwrap().document().unwrap().body().unwrap();
            root.set_inner_html(""); // Clear existing content

            let mut component = route.component.render();
            root.append_child(&component).unwrap();
        } else {
            // Handle 404
            let root = window().unwrap().document().unwrap().body().unwrap();
            root.set_inner_html("404 - Page Not Found");
        }
    }
}

// Route struct to associate paths with components
struct Route {
    path: String,
    component: Box<dyn Component + Send + Sync>,
}
