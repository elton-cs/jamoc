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
    One,
    Zero,
}

#[derive(Component, Clone, Copy, Debug)]
struct InputA(Bit);

#[derive(Component, Clone, Copy, Debug)]
struct InputB(Bit);

#[derive(Component, Clone, Copy, Debug)]
struct Output(Bit);

#[derive(Component, Clone, Copy, Debug)]
enum Gate {
    AND,
}

#[derive(Bundle)]
struct GateBundle {
    marker: Gate,
    input_a: InputA,
    input_b: InputB,
    output: Output,
}

fn spawn_and_gate(mut commands: Commands) {
    let init_bit = Bit::Zero;
    let marker = Gate::AND;
    let input_a = InputA(init_bit.clone());
    let input_b = InputB(init_bit.clone());
    let output = Output(init_bit);

    for i in 1..=3 {
        commands.spawn(GateBundle {
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
            (Bit::Zero, Bit::Zero) => (Bit::Zero, Bit::One),
            (Bit::Zero, Bit::One) => (Bit::One, Bit::Zero),
            (Bit::One, Bit::Zero) => (Bit::One, Bit::One),
            (Bit::One, Bit::One) => (Bit::Zero, Bit::Zero),
        };
        input_a.0 = new_a;
        input_b.0 = new_b;
    }
}

fn process_inputs(mut query: Query<(&Gate, &InputA, &InputB, &mut Output)>) {
    fn process_and(input_a: &InputA, input_b: &InputB) -> Bit {
        let new_output = match (input_a.0, input_b.0) {
            (Bit::One, Bit::One) => Bit::One,
            _ => Bit::Zero,
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

fn print_and_gate(query: Query<(&Gate, &InputA, &InputB, &Output)>) {
    let mut num_gates = 1;
    for (gate, input_a, input_b, output) in query.iter() {
        info!(
            "{num_gates}: [{:?} | {:?}] ---|{:?}| ==> [{:?}]",
            input_a.0, input_b.0, gate, output.0
        );
        num_gates += 1;
    }
}
