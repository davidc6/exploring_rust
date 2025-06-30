use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Filename(pub String);

impl Deref for Filename {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
