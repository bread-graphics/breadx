// MIT/Apache2 License

#![cfg(feature = "sync_display")]

cfg_not_pl! {
    use std::sync::{self, TryLockError};
}

/// A wrapper around `std::sync::Mutex` when `parking_lot` is not enabled,
/// and `parking_lot::Mutex` when it is.
pub(crate) struct Mutex<T> {
    #[cfg(feature = "pl")]
    mutex: parking_lot::Mutex<T>,
    #[cfg(not(feature = "pl"))]
    mutex: sync::Mutex<T>,
}

/// A wrapper around `std::sync::RwLock` when `parking_lot` is not enabled,
/// and `parking_lot::RwLock` when it is.
pub(crate) struct RwLock<T> {
    #[cfg(feature = "pl")]
    rwlock: parking_lot::RwLock<T>,
    #[cfg(not(feature = "pl"))]
    rwlock: sync::RwLock<T>,
}

/* mutex guard type aliases */

cfg_pl! {
    pub(crate) type MutexGuard<'a, T> = parking_lot::MutexGuard<'a, T>;
    pub(crate) type RwLockReadGuard<'a, T> = parking_lot::RwLockReadGuard<'a, T>;
    pub(crate) type RwLockWriteGuard<'a, T> = parking_lot::RwLockWriteGuard<'a, T>;
}

cfg_not_pl! {
    pub(crate) type MutexGuard<'a, T> = sync::MutexGuard<'a, T>;
    pub(crate) type RwLockReadGuard<'a, T> = sync::RwLockReadGuard<'a, T>;
    pub(crate) type RwLockWriteGuard<'a, T> = sync::RwLockWriteGuard<'a, T>;
}

cfg_pl! {
    impl<T> Mutex<T> {
        pub(crate) fn new(t: T) -> Self {
            Self {
                mutex: parking_lot::Mutex::new(t),
            }
        }

        pub(crate) fn lock(&self) -> MutexGuard<'_, T> {
            self.mutex.lock()
        }

        pub(crate) fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
            self.mutex.try_lock()
        }

        pub(crate) fn get_mut(&mut self) -> &mut T {
            self.mutex.get_mut()
        }
    }

    impl<T> RwLock<T> {
        pub(crate) fn new(t: T) -> Self {
            Self {
                rwlock: parking_lot::RwLock::new(t),
            }
        }

        pub(crate) fn read(&self) -> RwLockReadGuard<'_, T> {
            self.rwlock.read()
        }

        pub(crate) fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
            self.rwlock.try_read()
        }

        pub(crate) fn write(&self) -> RwLockWriteGuard<'_, T> {
            self.rwlock.write()
        }

        pub(crate) fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
            self.rwlock.try_write()
        }
    }
}

cfg_not_pl! {
    // macros to help bypass the poison guard

    macro_rules! lock {
        ($res: expr) => {
            match $res {
                Ok(guard) => guard,
                Err(poison) => {
                    // we don't care about poisoning, just call
                    // the into_inner method to get the value
                    // out of the poisoned state
                    poison.into_inner()
                }
            }
        }
    }

    macro_rules! try_lock {
        ($res: expr) => {
            match $res {
                Ok(guard) => Some(guard),
                Err(TryLockError::WouldBlock) => None,
                Err(TryLockError::Poisoned(poison)) => {
                    Some(poison.into_inner())
                }
            }
        }
    }

    impl<T> Mutex<T> {
        pub(crate) fn new(t: T) -> Self {
            Self {
                mutex: sync::Mutex::new(t),
            }
        }

        pub(crate) fn lock(&self) -> MutexGuard<'_, T> {
            lock!(self.mutex.lock())
        }

        pub(crate) fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
            try_lock!(self.mutex.try_lock())
        }

        pub(crate) fn get_mut(&self) -> &mut T {
            self.mutex.get_mut()
        }
    }

    impl<T> RwLock<T> {
        pub(crate) fn new(t: T) -> Self {
            Self {
                rwlock: sync::RwLock::new(t),
            }
        }

        pub(crate) fn read(&self) -> RwLockReadGuard<'_, T> {
            lock!(self.rwlock.read())
        }

        pub(crate) fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
            try_lock!(self.rwlock.try_read())
        }

        pub(crate) fn write(&self) -> RwLockWriteGuard<'_, T> {
            lock!(self.rwlock.write())
        }

        pub(crate) fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
            try_lock!(self.rwlock.try_write())
        }
    }
}
