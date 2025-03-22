use std::{ops::Deref, thread::Scope};

#[derive(PartialEq, Debug)]
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
    use crate::validation::TicketProgress;

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
}
