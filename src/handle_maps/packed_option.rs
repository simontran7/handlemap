use core::{fmt, mem};

/// Types that have a reserved value which can't be created any other way.
pub trait ReservedValue {
    /// Create an instance of the reserved value.
    fn reserved() -> Self;
    /// Checks whether value is the reserved one.
    fn is_reserved(&self) -> bool;
}

/// Packed representation of `Option<T>`.
///
/// This is a wrapper around a `T`, using `T::reserved` to represent `None`.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(transparent)]
pub struct PackedOption<T: ReservedValue>(T);

impl<T: ReservedValue> PackedOption<T> {
    /// Returns `true` if the packed option is a `None` value.
    pub fn is_none(&self) -> bool {
        self.0.is_reserved()
    }

    /// Returns `true` if the packed option is a `Some` value.
    pub fn is_some(&self) -> bool {
        !self.0.is_reserved()
    }

    /// Expand the packed option into a normal `Option`.
    pub fn expand(self) -> Option<T> {
        if self.is_none() { None } else { Some(self.0) }
    }

    /// Maps a `PackedOption<T>` to `Option<U>` by applying a function to a contained value.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
        self.expand().map(f)
    }

    /// Unwrap a packed `Some` value or panic.
    #[track_caller]
    pub fn unwrap(self) -> T {
        self.expand().unwrap()
    }

    /// Unwrap a packed `Some` value or panic with a message.
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        self.expand().expect(msg)
    }

    /// Takes the value out of the packed option, leaving `None` in its place.
    pub fn take(&mut self) -> Option<T> {
        mem::replace(self, None.into()).expand()
    }
}

impl<T: ReservedValue> Default for PackedOption<T> {
    /// Create a default packed option representing `None`.
    fn default() -> Self {
        Self(T::reserved())
    }
}

impl<T: ReservedValue> From<T> for PackedOption<T> {
    /// Convert `t` into a packed `Some(t)`.
    fn from(t: T) -> Self {
        debug_assert!(
            !t.is_reserved(),
            "Can't make a PackedOption from the reserved value."
        );
        Self(t)
    }
}

impl<T: ReservedValue> From<Option<T>> for PackedOption<T> {
    /// Convert an option into its packed equivalent.
    fn from(opt: Option<T>) -> Self {
        match opt {
            None => Self::default(),
            Some(t) => t.into(),
        }
    }
}

impl<T: ReservedValue> From<PackedOption<T>> for Option<T> {
    fn from(packed: PackedOption<T>) -> Option<T> {
        packed.expand()
    }
}

impl<T: ReservedValue + fmt::Debug> fmt::Debug for PackedOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_none() {
            write!(f, "None")
        } else {
            write!(f, "Some({:?})", self.0)
        }
    }
}
