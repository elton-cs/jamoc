use bevy::prelude::*;

use crate::{
    base::{Bit, Input, Output},
    input_node::InputNode,
};

pub struct WirePlugin;
impl Plugin for WirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_wire_with)
            .add_systems(Update, (link_wire_input, process_wire, view_wire).chain());
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Wire;

#[derive(Component, Clone, Copy, Debug)]
pub struct ConnectionFrom(pub Entity);

#[derive(Bundle, Clone, Copy, Debug)]
pub struct WireBundle {
    pub marker: Wire,
    pub input: Input,
    pub connection_from: ConnectionFrom,
    pub output: Output,
}

fn spawn_wire_with(mut commands: Commands, query: Query<(Entity, &InputNode)>) {
    let init_bit = Bit(false);

    for (node, _) in query.iter() {
        commands.spawn(WireBundle {
            marker: Wire,
            input: Input(init_bit),
            connection_from: ConnectionFrom(node),
            output: Output(init_bit),
        });
    }
}

fn link_wire_input(
    mut wire_query: Query<(&Wire, &mut Input, &ConnectionFrom)>,
    node_query: Query<&Output>,
) {
    for (_, mut input, connection_from) in wire_query.iter_mut() {
        let new_input = node_query.get(connection_from.0).unwrap();
        input.0 = new_input.0;
    }
}

fn process_wire(mut query: Query<(&Wire, &Input, &mut Output)>) {
    for (_, input, mut output) in query.iter_mut() {
        output.0 = input.0;
    }
}

fn view_wire(query: Query<(&Wire, &Input, &Output)>) {
    for (_, input, output) in query.iter() {
        info!("Wire: {:?} -> {:?}", input.0 .0, output.0 .0);
    }
}
