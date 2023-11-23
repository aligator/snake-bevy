use bevy::prelude::*;

use crate::game;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Resource, Default)]
pub struct LastTailPosition(pub(crate) Option<game::components::Position>);
