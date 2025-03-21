use std::ops::Deref;

#[derive(PartialEq)]
enum TicketProgress {
    Todo,
    InProgress { assigned_to: String },
    Done,
}

impl TicketProgress {
    fn is_done(&self) -> bool {
        match self {
            TicketProgress::Done => true,
            // catch-all case since we don't care about other variants
            _ => false,
        }
    }
}

#[derive(PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: TicketProgress,
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
            status: TicketProgress::Todo,
        }
    }
}

#[derive(Debug, PartialEq)]
enum TicketCreationError {
    Title(String),
}

impl Ticket {
    fn new(
        title: String,
        description: String,
        status: TicketProgress,
    ) -> Result<Self, TicketCreationError> {
        if title.is_empty() {
            return Err(TicketCreationError::Title("Title cannot be empty".into()));
        }

        if title.len() > 50 {
            return Err(TicketCreationError::Title(
                "Title is too long, maximum length is 50 characters".into(),
            ));
        }

        Ok(Self {
            title,
            description,
            status,
        })
    }

    fn assigned_to(&self) -> Option<&str> {
        if let TicketProgress::InProgress { assigned_to } = &self.status {
            return Some(assigned_to);
        }

        None
    }

    fn status(&self) {
        match &self.status {
            TicketProgress::InProgress { assigned_to } => println!("Assigned to {assigned_to}"),
            TicketProgress::Todo => println!("Ticket needs to be done"),
            TicketProgress::Done => println!("Ticket is done"),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

#[cfg(test)]
mod validation_tests {
    use std::any::{Any, TypeId};

    use crate::validation::TicketCreationError;

    use super::Ticket;

    #[test]
    // #[should_panic(expected = "Title cannot be empty")]
    fn ticket_validation_works() {
        let ticket = Ticket::new(
            "".into(),
            "desc".into(),
            super::TicketProgress::InProgress {
                assigned_to: "Bob".into(),
            },
        );

        assert!(ticket == Err(TicketCreationError::Title("Title cannot be empty".into())));
    }

    #[test]
    fn ticket_returns_title() {
        let ticket = Ticket::new(
            "Title".into(),
            "Desc".into(),
            crate::validation::TicketProgress::Done,
        );

        assert_eq!(TypeId::of::<str>(), ticket.unwrap().title().type_id());
    }

    #[test]
    fn ticket_size() {
        assert_eq!(size_of::<Ticket>(), 72);
    }

    #[test]
    fn deref_title() {
        let ticket = Ticket::new(
            "   Title   ".into(),
            "Desc".into(),
            crate::validation::TicketProgress::Todo,
        );

        // anti-pattern for demo purposes only
        assert_eq!(*ticket.unwrap(), *"Title");
    }

    #[test]
    fn from_works() {
        let ticket = Ticket::from("Title, Description, Status".to_owned());
        assert_eq!(ticket.title(), "Title");
    }
}
