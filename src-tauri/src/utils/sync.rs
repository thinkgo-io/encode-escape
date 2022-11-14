use std::sync::Arc;
use std::sync::Mutex;

pub fn into_arc_mutex<T>(item: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(item))
}

pub fn into_arc<T>(item: T) -> Arc<T> {
    Arc::new(item)
}
