use std::collections::HashMap;

struct datastore {
    store: Vec<u32>,
}

impl datastore {
    fn add(&mut self, data: u32) {
        self.store.push(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_add_mutex_lock_drop_gotcha() {
        let datastore = Arc::new(Mutex::new(datastore { store: vec![] }));
        for i in 0..3 {
            let d = datastore.clone();
            thread::spawn(move || {
                let len = d.lock().unwrap().store.len(); // lock dropped as not assign
                thread::sleep((Duration::from_millis(rand::thread_rng().gen_range(0, 10))));
                d.lock().unwrap().store.push(len as u32 + 1); // lock dropped as not assign
            });
        }
        thread::sleep(Duration::from_millis(200));
        print!("{:?}", datastore.lock().unwrap().store);
    }

    #[test]
    fn test_add_mutex() {
        let datastore = Arc::new(Mutex::new(datastore { store: vec![] }));
        for i in 0..3 {
            let d = datastore.clone();
            thread::spawn(move || {
                // assign lock here as not assign causes immediate drop
                let mut mx = d.lock().unwrap();
                let len = mx.store.len();
                thread::sleep((Duration::from_millis(rand::thread_rng().gen_range(0, 10))));
                mx.store.push(len as u32 + 1);
            }); // lock is dropped
        }
        thread::sleep(Duration::from_millis(200));
        print!("{:?}", datastore.lock().unwrap().store);
    }

    #[test]
    fn test_add_channel() {
        let datastore = datastore { store: vec![] };
    }
}
