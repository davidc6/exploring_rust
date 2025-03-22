use std::ops::Deref;

#[derive(Clone)]
struct TicketStore {
    tickets: Vec<Ticket>,
}

impl TicketStore {
    fn new() -> Self {
        Self { tickets: vec![] }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.iter()
    }
}

// impl Iterator for TicketStore {
//     type Item = Ticket;

//     fn next(&mut self) -> Option<Self::Item> {
//         let a = self.tickets.iter().next();
//         a.cloned()
//     }
// }

// Consuming (self) iterator, returns owned value
// Downside: original collection can no longer be used
impl IntoIterator for TicketStore {
    type Item = Ticket;
    type IntoIter = std::vec::IntoIter<Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_iter()
    }
}

/// Lifetimes are essentially labels that tell the compiler
/// how long a reference is valid for. The name how long
/// a reference is valid.
///
/// Lifetime of a reference is constrained by the scope of
/// the value it refers to.
///
/// Rus compiler makes sure that the reference of a value
/// is not used once the value is dropped (dangling pointers, use-after-free).
///
/// Lifetimes naming is important when it comes to multiple
/// references.
impl<'a> IntoIterator for &'a TicketStore {
    // This is the type to iterate over
    // In this example it's &Ticket
    type Item = &'a Ticket;
    // The iterator type returned by the into_iter() method
    // In this example it's immutable slice iterator over tickets
    type IntoIter = std::slice::Iter<'a, Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}

#[derive(PartialEq, Debug, Clone)]
enum TicketProgress {
    Todo,
    InProgress { assigned_to: String },
    Done,
}

#[derive(thiserror::Error, Debug, PartialEq)]
#[error("Ticket status is not known")]
struct StatusParseError {
    invalid_status: String,
}

impl TryFrom<&str> for TicketProgress {
    type Error = StatusParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // important to use as_str() here to extract &str slice from String
        match value.to_lowercase().as_str() {
            "todo" => Ok(TicketProgress::Todo),
            "inprogress" => Ok(TicketProgress::InProgress {
                assigned_to: "".to_owned(),
            }),
            "done" => Ok(TicketProgress::Done),
            _ => Err(StatusParseError {
                invalid_status: value.to_string(),
            }),
        }
    }
}

impl TryFrom<String> for TicketProgress {
    type Error = StatusParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
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

#[derive(PartialEq, Clone, Debug)]
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

#[derive(thiserror::Error, Debug, PartialEq)]
enum TicketCreationError {
    #[error("Title cannot be empty")]
    TitleEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleLength,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionEmpty,
}

impl Ticket {
    fn new(
        title: String,
        description: String,
        status: TicketProgress,
    ) -> Result<Self, TicketCreationError> {
        if title.is_empty() {
            return Err(TicketCreationError::TitleEmpty);
        }

        if title.len() > 50 {
            return Err(TicketCreationError::TitleLength);
        }

        if description.len() > 500 {
            return Err(TicketCreationError::DescriptionEmpty);
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
    use crate::validation::{TicketProgress, TicketStore};

    use super::{Ticket, TicketCreationError};
    use std::any::{Any, TypeId};

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

        assert!(ticket == Err(TicketCreationError::TitleEmpty));
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

    #[test]
    fn test_try_from_string() {
        let status = TicketProgress::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, TicketProgress::Todo);

        let status = TicketProgress::try_from("inproGress".to_string()).unwrap();
        assert_eq!(
            status,
            TicketProgress::InProgress {
                assigned_to: "".to_string()
            }
        );

        let status = TicketProgress::try_from("Done".to_string()).unwrap();
        assert_eq!(status, TicketProgress::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = TicketProgress::try_from("todo").unwrap();
        assert_eq!(status, TicketProgress::Todo);

        let status = TicketProgress::try_from("inprogress").unwrap();
        assert_eq!(
            status,
            TicketProgress::InProgress {
                assigned_to: "".to_owned()
            }
        );

        let status = TicketProgress::try_from("done").unwrap();
        assert_eq!(status, TicketProgress::Done);
    }

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: "Title".into(),
            description: "Description".into(),
            status: TicketProgress::Todo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: "Title 2".into(),
            description: "Description 2".into(),
            status: TicketProgress::InProgress {
                assigned_to: "".to_owned(),
            },
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }

    #[test]
    fn add_ticket_2() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: "Title".into(),
            description: "Description".into(),
            status: TicketProgress::Todo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: "Title 2".into(),
            description: "Description 2".into(),
            status: TicketProgress::InProgress {
                assigned_to: "".to_owned(),
            },
        };
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = store.iter().collect();
        assert_eq!(tickets, tickets2);
    }

    #[test]
    fn add_ticket_3() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: "Title 3".into(),
            description: "Description 3".into(),
            status: TicketProgress::Todo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: "Title 4".into(),
            description: "Description 4".into(),
            status: TicketProgress::InProgress {
                assigned_to: "".to_owned(),
            },
        };
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = (&store).into_iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
