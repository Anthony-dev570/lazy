use std::fmt::{Debug, Formatter};
use crate::lazy::Lazy;

///A lazy object that also contains an initializer. This is used to be wrapped around a standard lazy, but gives you the ability to call the 'get' function without having to keep supplying an initializer.
pub struct LazyObject<U, I> {
    lazy: Lazy<U, I>,
    initializer: Box<dyn FnMut(&mut U) -> I>
}

impl <U, I> Debug for LazyObject<U, I> where U: Debug, I: Debug{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.lazy))
    }
}

impl <U, I> LazyObject<U, I> {
    pub fn new(lazy: Lazy<U, I>, initializer: Box<dyn FnMut(&mut U) -> I>) -> Self {
        Self {
            lazy,
            initializer,
        }
    }

    ///Determines if this lazy object has been initialized.
    pub fn is_initialized(&self) -> bool {
        self.lazy.is_initialized()
    }

    ///Determines if this lazy object has yet to be initialized.
    pub fn is_uninitialized(&self) -> bool {
        self.lazy.is_uninitialized()
    }

    ///Manually initialize this lazy object. If the object was already initialized, then this is ignored.
    pub fn initialize(&mut self) {
        self.lazy.initialize(&mut self.initializer);
    }

    ///Attempts to initialize this lazy object. This will fail if the object has already been initialized. This will initialize the lazy if it has yet to be initialized.
    pub fn get(&mut self) -> &mut I {
        self.initialize();
        self.try_initialized().unwrap()
    }

    ///Tries to return the inner initialized object. This will fail if the object has yet to be initialized.
    pub fn try_initialized(&mut self) -> Option<&mut I> {
        self.lazy.try_initialized()
    }

    ///Tries to return the inner uninitialized object. This will fail if the object has been initialized.
    pub fn try_uninitialized(&mut self) -> Option<&mut U> {
        self.lazy.try_uninitialized()
    }
}