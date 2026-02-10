#![forbid(unsafe_code)]

//! Optional value type inspired by JPA Optional.

/// Optional value wrapper.
#[derive(Debug, Clone, Default)]
pub struct Optional<T>(Option<T>);

impl<T> Optional<T> {
    /// Create an empty Optional.
    pub fn empty() -> Self {
        Self(None)
    }

    /// Create an Optional with a value.
    pub fn of(value: T) -> Self {
        Self(Some(value))
    }

    /// Create an Optional from an Option.
    pub fn of_nullable(value: Option<T>) -> Self {
        Self(value)
    }

    /// Return true if a value is present.
    pub fn is_present(&self) -> bool {
        self.0.is_some()
    }

    /// Return true if no value is present.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    /// Get a reference to the value.
    pub fn get(&self) -> Option<&T> {
        self.0.as_ref()
    }

    /// Execute a function if the value is present.
    pub fn if_present<F>(&self, f: F)
    where
        F: FnOnce(&T),
    {
        if let Some(value) = &self.0 {
            f(value);
        }
    }

    /// Map the optional value.
    pub fn map<U, F>(self, f: F) -> Optional<U>
    where
        F: FnOnce(T) -> U,
    {
        Optional(self.0.map(f))
    }

    /// Return the value or a default.
    pub fn unwrap_or(self, default: T) -> T {
        self.0.unwrap_or(default)
    }

    /// Convert into the underlying Option.
    pub fn into_option(self) -> Option<T> {
        self.0
    }
}

impl<T> From<Option<T>> for Optional<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

impl<T> From<Optional<T>> for Option<T> {
    fn from(value: Optional<T>) -> Self {
        value.0
    }
}
