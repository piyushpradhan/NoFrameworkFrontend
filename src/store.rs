use spin::{Mutex, MutexGuard};
use std::sync::OnceLock;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

// Store concrete types rather than trait objects
static GLOBALS_LIST: OnceLock<Mutex<HashMap<TypeId, &'static (dyn Any + Send + Sync)>>> =
    OnceLock::new();

pub fn get<T>() -> MutexGuard<'static, T>
where
    T: 'static + Default + Send + Sync,
{
    let globals = GLOBALS_LIST.get_or_init(|| Mutex::new(HashMap::new()));
    {
        let mut globals = globals.lock();
        let type_id = TypeId::of::<T>();

        if let Some(value) = globals.get(&type_id) {
            // Safe downcast from &dyn Any to &Mutex<T>
            let mutex = value
                .downcast_ref::<Mutex<T>>()
                .expect("Type mismatch in global storage");
            return mutex.lock();
        }

        // Initialize new value if not present
        let new_mutex = Box::new(Mutex::new(T::default()));
        let leaked_mutex: &'static Mutex<T> = Box::leak(new_mutex);
        globals.insert(type_id, leaked_mutex);
    }
    // Recursive call to get the newly inserted value
    get()
}

pub fn set<T>(data: T)
where
    T: 'static + Default + Send + Sync,
{
    let globals = GLOBALS_LIST.get_or_init(|| Mutex::new(HashMap::new()));
    {
        let mut globals = globals.lock();
        let type_id = TypeId::of::<T>();

        // Create and insert new mutex
        let new_mutex = Box::new(Mutex::new(data));
        let leaked_mutex: &'static Mutex<T> = Box::leak(new_mutex);
        globals.insert(type_id, leaked_mutex);
    }
}

pub fn remove<T>()
where
    T: 'static + Default + Send + Sync,
{
    if let Some(globals) = GLOBALS_LIST.get() {
        let mut globals = globals.lock();
        globals.remove(&TypeId::of::<T>());
    }
}
