use bevy::prelude::*;

use crate::{
    base::{Bit, Input, Output},
    clock::Clock,
};

pub struct InputNodePlugin;
impl Plugin for InputNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_input_node).add_systems(
            Update,
            (toggle_input_node, process_node, view_input_node).chain(),
        );
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct InputNode;

#[derive(Bundle, Clone, Copy, Debug)]
pub struct InputNodeBundle {
    pub marker: InputNode,
    pub input: Input,
    pub output: Output,
}

fn spawn_input_node(mut commands: Commands) {
    let init_bit = Bit(false);

    commands.spawn(InputNodeBundle {
        marker: InputNode,
        input: Input(init_bit),
        output: Output(init_bit),
    });
}

fn toggle_input_node(
    time: Res<Time>,
    mut clock: ResMut<Clock>,
    mut query: Query<(&InputNode, &mut Input)>,
) {
    clock.timer.tick(time.delta());
    for (_, mut input) in query.iter_mut() {
        if clock.timer.finished() {
            input.0 = Bit(!input.0 .0);
        }
    }
}

fn process_node(mut query: Query<(&InputNode, &Input, &mut Output)>) {
    for (_, input, mut output) in query.iter_mut() {
        output.0 = input.0;
    }
}

fn view_input_node(query: Query<(&InputNode, &Input, &Output)>) {
    for (_, input, output) in query.iter() {
        info!("InputNode: {:?} -> {:?}", input.0 .0, output.0 .0);
    }
}
