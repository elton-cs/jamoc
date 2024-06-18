use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Bit(pub bool);

#[derive(Component, Clone, Copy, Debug)]
pub struct Input(pub Bit);

#[derive(Component, Clone, Copy, Debug)]
pub struct Output(pub Bit);
