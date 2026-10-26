use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::RwLock;

#[derive(Default)]
pub struct PlanetPropertyRegistry {
    storage: HashMap<TypeId, RwLock<Box<dyn Any + Send + Sync>>>,
}

impl PlanetPropertyRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T: 'static + Send + Sync>(&mut self, value: T) -> Option<T> {
        self.storage
            .insert(TypeId::of::<T>(), RwLock::new(Box::new(value)))
            .and_then(|rwlock| {
                rwlock
                    .into_inner()
                    .ok()
                    .and_then(|boxed| boxed.downcast::<T>().ok().map(|b| *b))
            })
    }

    pub fn get<T: 'static + Send + Sync>(&self) -> Option<PropertyGuard<'_, T>> {
        self.storage
            .get(&TypeId::of::<T>())
            .map(|rwlock| PropertyGuard {
                guard: rwlock.read().unwrap(),
                _marker: std::marker::PhantomData,
            })
    }

    pub fn get_mut<T: 'static + Send + Sync>(&self) -> Option<PropertyGuardMut<'_, T>> {
        self.storage
            .get(&TypeId::of::<T>())
            .map(|rwlock| PropertyGuardMut {
                guard: rwlock.write().unwrap(),
                _marker: std::marker::PhantomData,
            })
    }

    pub fn remove<T: 'static + Send + Sync>(&mut self) -> Option<T> {
        self.storage
            .remove(&TypeId::of::<T>())
            .and_then(|rwlock| {
                rwlock
                    .into_inner()
                    .ok()
                    .and_then(|boxed| boxed.downcast::<T>().ok().map(|b| *b))
            })
    }

    pub fn has<T: 'static + Send + Sync>(&self) -> bool {
        self.storage.contains_key(&TypeId::of::<T>())
    }
}

pub struct PropertyGuard<'a, T: ?Sized> {
    guard: std::sync::RwLockReadGuard<'a, Box<dyn Any + Send + Sync>>,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<T: 'static + Send + Sync> Deref for PropertyGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        (**self.guard).downcast_ref::<T>().unwrap()
    }
}

pub struct PropertyGuardMut<'a, T: ?Sized> {
    guard: std::sync::RwLockWriteGuard<'a, Box<dyn Any + Send + Sync>>,
    _marker: std::marker::PhantomData<&'a mut T>,
}

impl<T: 'static + Send + Sync> Deref for PropertyGuardMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        (**self.guard).downcast_ref::<T>().unwrap()
    }
}

impl<T: 'static + Send + Sync> DerefMut for PropertyGuardMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        (**self.guard).downcast_mut::<T>().unwrap()
    }
}