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

pub fn set<T>(data: T)
where
    T: 'static + Default + Send + core::marker::Sync,
{
    {
        let mut globals = GLOBALS_LIST.lock();
        let type_id = TypeId::of::<T>();
        let mut found_index = None;

        for (index, &stored_type_id) in globals.iter().enumerate() {
            if stored_type_id.0 == type_id {
                found_index = Some(index);
                break;
            }
        }

        if let Some(index) = found_index {
            globals.pop_back();
        }

        let updated_mutex = Box::new(Mutex::new(data));
        let leaked_mutex = Box::leak(updated_mutex);
        globals.push_front((type_id, leaked_mutex));
    }
}
