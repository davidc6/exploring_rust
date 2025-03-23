use std::ops::Deref;

#[derive(Clone)]
struct TicketStore {
    tickets: Vec<Ticket>,
    ticket_counter: u32,
}

#[derive(Clone)]
struct DraftTicket {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TicketId(u32);

#[derive(PartialEq, Clone, Debug)]
pub struct Ticket {
    id: TicketId,
    title: String,
    description: String,
    status: TicketProgress,
}

impl From<DraftTicket> for Ticket {
    fn from(value: DraftTicket) -> Self {
        let DraftTicket { title, description } = value;

        Ticket {
            id: TicketId(1),
            title,
            description,
            status: TicketProgress::Todo,
        }
    }
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
            id: TicketId(s.first().unwrap_or(&"1").parse::<u32>().unwrap()),
            title: s.get(1).unwrap_or(&"").to_string().trim().to_string(),
            description: s.get(2).unwrap_or(&"").to_string(),
            status: TicketProgress::Todo,
        }
    }
}

impl TicketStore {
    fn new() -> Self {
        Self {
            tickets: vec![],
            ticket_counter: 0,
        }
    }

    pub fn add_ticket<T: Into<Ticket>>(&mut self, ticket: T) -> TicketId {
        let mut t = ticket.into();

        self.ticket_counter += 1;
        let c = self.ticket_counter;
        t.id = TicketId(c);

        self.tickets.push(t.clone());
        TicketId(c)
    }

    pub fn add_draft<T: Into<Ticket>>(&mut self, draft_ticket: T) -> TicketId {
        // let t = Ticket::from(draft_ticket);
        // let id = t.id;
        let mut t: Ticket = draft_ticket.into();

        self.ticket_counter += 1;
        t.id = TicketId(self.ticket_counter);

        self.tickets.push(t.clone());
        TicketId(self.ticket_counter)
    }

    pub fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.iter()
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        println!("TICKETS {:?}", self.tickets);
        self.tickets.iter().find(|ticket| ticket.id == id)
    }

    /// Get all tickets that have Todo status from the available tickets
    /// impl trait - return a type without specifying the name
    pub fn todos(&self) -> Vec<&Ticket> {
        self.tickets
            .iter()
            .filter(|ticket| ticket.status == TicketProgress::Todo)
            .collect()
    }

    pub fn in_progress(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.iter().filter(|ticket| {
            ticket.status
                == TicketProgress::InProgress {
                    assigned_to: "".to_owned(),
                }
        })
    }
}

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
        id: String,
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
            id: TicketId(1),
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
    use crate::validation::{DraftTicket, TicketId, TicketProgress, TicketStore};

    use super::{Ticket, TicketCreationError};
    use std::any::{Any, TypeId};

    #[test]
    // #[should_panic(expected = "Title cannot be empty")]
    fn ticket_validation_works() {
        let ticket = Ticket::new(
            "".into(),
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
            "1".into(),
            "Title".into(),
            "Desc".into(),
            crate::validation::TicketProgress::Done,
        );

        assert_eq!(TypeId::of::<str>(), ticket.unwrap().title().type_id());
    }

    #[test]
    fn ticket_size() {
        assert_eq!(size_of::<Ticket>(), 80);
    }

    #[test]
    fn deref_title() {
        let ticket = Ticket::new(
            "id".into(),
            "   Title   ".into(),
            "Desc".into(),
            crate::validation::TicketProgress::Todo,
        );

        // anti-pattern for demo purposes only
        assert_eq!(*ticket.unwrap(), *"Title");
    }

    #[test]
    fn from_works() {
        let ticket = Ticket::from("1, Title, Description, Status".to_owned());
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
            id: TicketId(1),
            title: "Title".into(),
            description: "Description".into(),
            status: TicketProgress::Todo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            id: TicketId(2),
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
            id: TicketId(1),
            title: "Title".into(),
            description: "Description".into(),
            status: TicketProgress::Todo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            id: TicketId(2),
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
            id: TicketId(3),
            title: "Title 3".into(),
            description: "Description 3".into(),
            status: TicketProgress::Todo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            id: TicketId(4),
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

    #[test]
    fn todos() {
        let mut store = TicketStore::new();

        let ticket_todo = Ticket {
            id: TicketId(1),
            title: "Title 3".into(),
            description: "Description 3".into(),
            status: TicketProgress::Todo,
        };
        store.add_ticket(ticket_todo.clone());

        let ticket = Ticket {
            id: TicketId(1),
            title: "Title 4".into(),
            description: "Description 4".into(),
            status: TicketProgress::InProgress {
                assigned_to: "".to_owned(),
            },
        };
        store.add_ticket(ticket);

        let todos: Vec<&Ticket> = store.todos();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0], &ticket_todo);
    }

    #[test]
    fn in_progress() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            id: TicketId(1),
            title: "Title".into(),
            description: "Description".into(),
            status: TicketProgress::Todo,
        };
        store.add_ticket(ticket);

        let in_progress = Ticket {
            id: TicketId(2),
            title: "Title 2".into(),
            description: "Description 2".into(),
            status: TicketProgress::InProgress {
                assigned_to: "".to_owned(),
            },
        };
        store.add_ticket(in_progress.clone());

        let in_progress_tickets: Vec<&Ticket> = store.in_progress().collect();
        assert_eq!(in_progress_tickets.len(), 1);
        assert_eq!(in_progress_tickets[0], &in_progress);
    }

    #[test]
    fn into_ticket_works() {
        let mut store = TicketStore::new();

        let draft1 = DraftTicket {
            title: "Title 1".into(),
            description: "desc 1".into(),
        };
        let id1 = store.add_draft(draft1.clone());
        let ticket1 = store.get(id1).unwrap();
        assert_eq!(draft1.title, ticket1.title);
        assert_eq!(draft1.description, ticket1.description);
        assert_eq!(ticket1.status, TicketProgress::Todo);

        let draft2 = DraftTicket {
            title: "Title 2".into(),
            description: "Desc 2".into(),
        };
        let id2 = store.add_ticket(draft2);
        let ticket2 = store.get(id2).unwrap();

        assert_ne!(id1, id2);
    }
}
