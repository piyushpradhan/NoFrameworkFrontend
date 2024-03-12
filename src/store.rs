use spin::{Mutex, MutexGuard};
use std::{
    any::{Any, TypeId},
    collections::LinkedList,
};

static GLOBALS_LIST: Mutex<LinkedList<(TypeId, &'static Mutex<dyn Any + Send + Sync>)>> =
    Mutex::new(LinkedList::new());

pub fn get<T>() -> MutexGuard<'static, T>
where
    T: 'static + Default + Send + core::marker::Sync,
{
    {
        let mut globals = GLOBALS_LIST.lock();
        let id = TypeId::of::<T>();
        let p = globals.iter().find(|&r| r.0 == id);
        if let Some(v) = p {
            let m = unsafe { &*(v.1 as *const Mutex<dyn Any + Send + Sync> as *const Mutex<T>) };
            return m.lock();
        }
        let v = Box::new(Mutex::new(T::default()));
        let handle = Box::leak(v);
        globals.push_front((id, handle));
    }
    get()
}
