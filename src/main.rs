use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_input_node_and_wire, spawn_timer).chain())
        .add_systems(
            Update,
            (
                toggle_input_node,
                process_node,
                link_wire_input,
                process_wire,
                view_input_node,
                view_wire,
            )
                .chain(),
        )
        .run();
}

#[derive(Component, Clone, Copy, Debug)]
struct Bit(bool);

#[derive(Component, Clone, Copy, Debug)]
struct Input(Bit);

#[derive(Component, Clone, Copy, Debug)]
struct Output(Bit);

#[derive(Component, Clone, Copy, Debug)]
struct ConnectionFrom(Entity);

#[derive(Component, Clone, Copy, Debug)]
struct _ConnectionTo(Entity);

#[derive(Component, Clone, Copy, Debug)]
struct Wire;

#[derive(Bundle, Clone, Copy, Debug)]
struct WireBundle {
    marker: Wire,
    input: Input,
    connection_from: ConnectionFrom,
    output: Output,
}

#[derive(Component, Clone, Copy, Debug)]
struct InputNode;

#[derive(Bundle, Clone, Copy, Debug)]
struct InputNodeBundle {
    marker: InputNode,
    input: Input,
    output: Output,
}

#[derive(Resource)]
struct Clock {
    timer: Timer,
}

fn spawn_input_node_and_wire(mut commands: Commands) {
    let init_bit = Bit(false);

    let node = commands
        .spawn(InputNodeBundle {
            marker: InputNode,
            input: Input(init_bit),
            output: Output(init_bit),
        })
        .id();

    let _wire = commands
        .spawn(WireBundle {
            marker: Wire,
            input: Input(init_bit),
            connection_from: ConnectionFrom(node),
            output: Output(init_bit),
        })
        .id();
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

fn spawn_timer(mut commands: Commands) {
    commands.insert_resource(Clock {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
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
