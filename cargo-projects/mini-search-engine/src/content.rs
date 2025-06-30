use std::ops::Deref;

pub struct Content(pub String);

impl Deref for Content {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
