use spin::{Mutex, MutexGuard};

pub struct Locked<T> {
    inner: Mutex<T>
}

impl<T> Locked<T> {
    pub const fn new(value: T) -> Self {
        return Locked { inner: Mutex::new(value) }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.inner.lock()
    }
}
