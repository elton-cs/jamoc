use bevy::prelude::*;

pub struct ClockPlugin;
impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_timer);
    }
}

#[derive(Resource)]
pub struct Clock {
    pub timer: Timer,
}

fn spawn_timer(mut commands: Commands) {
    commands.insert_resource(Clock {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
}
