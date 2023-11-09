use bevy::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Down,
    Right,
    Up,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
        }
    }
}

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}
