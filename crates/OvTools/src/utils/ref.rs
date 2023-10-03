use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock, RwLockReadGuard},
};

#[derive(Debug)]
pub struct Ref<T: ?Sized>(pub Arc<RwLock<T>>);

unsafe impl<T: ?Sized> Send for Ref<T> {}

unsafe impl<T: ?Sized> Sync for Ref<T> {}

// #[unstable(feature = "dispatch_from_dyn", issue = "none")]
// impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Ref<U>> for Ref<T> {}

impl<T> Ref<T> {
    pub fn new(value: T) -> Self {
        Ref(Arc::new(RwLock::new(value)))
    }

    pub fn inner(&self) -> Arc<RwLock<T>> {
        self.0.clone()
    }
}

impl<T: ?Sized> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Ref(self.0.clone())
    }
}

impl<T: ?Sized> Deref for Ref<T> {
    type Target = Arc<RwLock<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Ref<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
