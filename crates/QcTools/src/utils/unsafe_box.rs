pub struct UnsafeBox<T>(*const T);

unsafe impl<T> Send for UnsafeBox<T> {}
unsafe impl<T> Sync for UnsafeBox<T> {}

impl<T> UnsafeBox<T> {
    pub fn new(inner: *const T) -> Self {
        Self(inner)
    }

    pub fn as_raw(&self) -> &*const T {
         &self.0 
    }
}
