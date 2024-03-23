use web_sys::window;

pub fn get_item(key: &str) -> String {
    let local_storage = window().unwrap().local_storage().unwrap();
    match local_storage {
        Some(storage) => {
            return storage.get_item(key).unwrap().unwrap_or(String::new());
        }
        None => {
            return String::new();
        }
    }
}

pub fn set_item(key: &str, value: &str) {
    let local_storage = window().unwrap().local_storage().unwrap();

    match local_storage {
        Some(storage) => {
            storage.set_item(key, value).unwrap();
        }
        None => {}
    }
}
