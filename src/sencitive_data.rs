use std::fmt::{Debug, Display};
use std::ops::Deref;

#[derive(PartialEq)]
pub(crate) struct SensitiveData<T>(T);

impl<T> Deref for SensitiveData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Debug for SensitiveData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SensitiveData").field(&"hidden").finish()
    }
}

impl<T> Display for SensitiveData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hidden")
    }
}

impl<T> From<T> for SensitiveData<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
