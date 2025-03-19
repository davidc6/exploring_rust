use std::ops::Deref;

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Deref for Ticket {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.title.trim()
    }
}

impl From<String> for Ticket {
    fn from(value: String) -> Self {
        let s: Vec<_> = value.split(',').collect();

        Ticket {
            title: s.first().unwrap_or(&"").to_string(),
            description: s.get(1).unwrap_or(&"").to_string(),
            status: s.get(2).unwrap_or(&"").to_string(),
        }
    }
}

impl Ticket {
    fn new(title: String, description: String, status: String) -> Self {
        if title.is_empty() {
            panic!("Title cannot be empty")
        }

        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes")
        }

        Self {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

#[cfg(test)]
mod validation_tests {
    use std::any::{Any, TypeId};

    use super::Ticket;

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn ticket_validation_works() {
        Ticket::new("".into(), "desc".into(), "open".into());
    }

    #[test]
    fn ticket_returns_title() {
        let ticket = Ticket::new("Title".into(), "Desc".into(), "Open".into());

        assert!(ticket.title() == "Title");
        assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
    }

    #[test]
    fn ticket_size() {
        assert_eq!(size_of::<Ticket>(), 72);
    }

    #[test]
    fn deref_title() {
        let ticket = Ticket::new("   Title   ".into(), "Desc".into(), "Open".into());

        // anti-pattern for demo purposes only
        assert_eq!(*ticket, *"Title");
    }

    #[test]
    fn from_works() {
        let ticket = Ticket::from("Title, Description, Status".to_owned());
        assert_eq!(ticket.title(), "Title");
    }
}
