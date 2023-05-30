use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::time::{Duration, Instant};
use std::clone::Clone;

pub struct SharedObjectError;

#[derive(Debug,Clone)]
pub struct SharedObject<T : Clone > {
    shared : Arc<RwLock<T>>,
}

impl <T:Clone>SharedObject<T> {

    pub fn new(obj : T) -> SharedObject<T>{
        SharedObject {shared : Arc::new(RwLock::new(obj.clone()))}
    }

    pub fn read(&mut self) -> T {
        let guard = self.shared.read().expect("Could not get RW lock");
        let result = guard.clone();
        result
    }

    pub fn write(&self, obj: T, dur : Option<Duration>) -> Result<(),SharedObjectError> {

        let mut result = Err(SharedObjectError);
        if let Some(dur) = dur {

            let now = Instant::now();

            while now.elapsed() <= dur {
                match self.shared.try_write() {
                    Ok(mut guard) => {
                        *guard = obj;
                        result = Ok(());
                        break;
                    },
                    Err(_err) => (),
                };
            }
        }
        // let mut obj_guard = self.shared.write().unwrap();
        // *stats_guard = _stats;
        result
    }

    pub fn shared(&self) -> RwLockReadGuard<T>{
        self.shared.read().expect("Could not get RW lock")
    }
}
