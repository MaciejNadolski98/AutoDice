use bevy::prelude::*;

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Battle,
    Manage,
}
