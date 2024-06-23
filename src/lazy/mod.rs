///Defines a lazy initializer. This is usable anywhere you have like data between two objects, but need to be able to initialize the data at a later time.
#[derive(Debug, Clone)]
pub enum Lazy<U, I> {
    ///Uninitialized state. This holds onto the data before the lazy initialization.
    Uninitialized(U),
    ///Initialized state. This holds onto the data after the lazy initialization.
    Initialized(I)
}

impl <U, I> Lazy<U, I> {
    ///Determines if this lazy object has been initialized.
    pub fn is_initialized(&self) -> bool {
        match self {
            Lazy::Uninitialized(_) => false,
            Lazy::Initialized(_) => true
        }
    }

    ///Determines if this lazy object has yet to be initialized.
    pub fn is_uninitialized(&self) -> bool {
        !self.is_initialized()
    }

    ///Manually initialize this lazy object. If the object was already initialized, then this is ignored.
    pub fn initialize<Init: FnMut(&mut U) -> I>(&mut self, mut initialize: Init) {
        match self {
            Lazy::Uninitialized(u) => {
                *self = Self::Initialized(initialize(u));
            }
            _ => {}
        }
    }

    ///Attempts to initialize this lazy object. This will fail if the object has already been initialized. This will initialize the lazy if it has yet to be initialized.
    pub fn get<Init: FnMut(&mut U) -> I>(&mut self, initialize: Init) -> &mut I {
        self.initialize(initialize);
        self.try_initialized().unwrap()
    }

    ///Tries to return the inner initialized object. This will fail if the object has yet to be initialized.
    pub fn try_initialized(&mut self) -> Option<&mut I> {
        if let Self::Initialized(i) = self {
            return Some(i);
        }
        None
    }

    ///Tries to return the inner uninitialized object. This will fail if the object has been initialized.
    pub fn try_uninitialized(&mut self) -> Option<&mut U> {
        if let Self::Uninitialized(u) = self {
            return Some(u);
        }
        None
    }
}