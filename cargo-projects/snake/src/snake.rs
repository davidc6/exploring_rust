use std::collections::LinkedList;

use crate::utils::{Directions, Position};

#[derive(Debug)]
pub struct Snake {
    pub position: Position,
    pub direction: Directions,
    pub body: LinkedList<Position>,
}

impl Snake {
    pub fn head_position(&self) -> &Position {
        return &self.position;
    }

    pub fn head_direction(&self) -> &Directions {
        return &self.direction;
    }

    // TODO
    fn increase_length() {}

    pub fn get_body(&self) -> &LinkedList<Position> {
        return &self.body;
    }
}
