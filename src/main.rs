use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_and_gate)
        .add_systems(
            Update,
            (cycle_inputs, process_inputs, print_and_gate).chain(),
        )
        .run();
}

#[derive(Component, Clone, Copy, Debug)]
enum Bit {
    I,
    O,
}

#[derive(Component, Clone, Copy, Debug)]
struct InputA(Bit);

#[derive(Component, Clone, Copy, Debug)]
struct PropagateA(Entity);

#[derive(Component, Clone, Copy, Debug)]
struct InputB(Bit);

#[derive(Component, Clone, Copy, Debug)]
struct PropagateB(Entity);

#[derive(Component, Clone, Copy, Debug)]
struct Output(Bit);

#[derive(Component, Clone, Copy, Debug)]
enum Gate {
    AND,
}

#[derive(Bundle)]
struct StartGate {
    marker: Gate,
    input_a: InputA,
    input_b: InputB,
    output: Output,
}

#[derive(Bundle)]
struct MidGate {
    marker: Gate,
    input_a: PropagateA,
    input_b: PropagateB,
    output: Output,
}

fn spawn_and_gate(mut commands: Commands) {
    let init_bit = Bit::O;
    let marker = Gate::AND;
    let input_a = InputA(init_bit.clone());
    let input_b = InputB(init_bit.clone());
    let output = Output(init_bit);

    for i in 1..=3 {
        commands.spawn(StartGate {
            marker: marker.clone(),
            input_a: input_a.clone(),
            input_b: input_b.clone(),
            output: output.clone(),
        });
    }
}

fn cycle_inputs(mut query: Query<(&mut InputA, &mut InputB)>) {
    for (mut input_a, mut input_b) in query.iter_mut() {
        let (new_a, new_b) = match (input_a.0, input_b.0) {
            (Bit::O, Bit::O) => (Bit::O, Bit::I),
            (Bit::O, Bit::I) => (Bit::I, Bit::O),
            (Bit::I, Bit::O) => (Bit::I, Bit::I),
            (Bit::I, Bit::I) => (Bit::O, Bit::O),
        };
        input_a.0 = new_a;
        input_b.0 = new_b;
    }
}

fn process_inputs(mut query: Query<(&Gate, &InputA, &InputB, &mut Output)>) {
    fn process_and(input_a: &InputA, input_b: &InputB) -> Bit {
        let new_output = match (input_a.0, input_b.0) {
            (Bit::I, Bit::I) => Bit::I,
            _ => Bit::O,
        };

        new_output
    }

    for (gate, input_a, input_b, mut output) in query.iter_mut() {
        let new_output = match gate {
            Gate::AND => process_and(input_a, input_b),
        };

        output.0 = new_output;
    }
}

fn print_and_gate(query: Query<(Entity, &Gate, &InputA, &InputB, &Output)>) {
    for (entity, gate, input_a, input_b, output) in query.iter() {
        info!(
            "{:?}: [{:?} | {:?}] = |{:?}| > [{:?}]",
            entity, input_a.0, input_b.0, gate, output.0
        );
    }
}
