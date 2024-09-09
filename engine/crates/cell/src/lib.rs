use std::{cell::UnsafeCell, marker::PhantomData};

pub struct UnsafePtrCell<'a, T>(*mut T, PhantomData<(&'a T, &'a UnsafeCell<T>)>);

impl<'a, T> UnsafePtrCell<'a, T> {
    pub fn from_ref(value: &'a T) -> Self {
        Self(std::ptr::from_ref(value).cast_mut(), PhantomData)
    }

    pub fn from_mut(value: &'a mut T) -> Self {
        Self(std::ptr::from_mut(value), PhantomData)
    }

    pub unsafe fn get(self) -> &'a T {
        &*self.0
    }

    pub unsafe fn get_mut(self) -> &'a mut T {
        &mut *self.0
    }
}

impl<'a, T> Clone for UnsafePtrCell<'a, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<'a, T> Copy for UnsafePtrCell<'a, T> { }

impl<'a, T> From<&'a T> for UnsafePtrCell<'a, T> {
    fn from(value: &'a T) -> Self {
        Self::from_ref(value)
    }
}

impl<'a, T> From<&'a mut T> for UnsafePtrCell<'a, T> {
    fn from(value: &'a mut T) -> Self {
        Self::from_mut(value)
    }
}
