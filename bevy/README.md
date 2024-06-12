# quickstart_ecs
- https://bevyengine.org/learn/quick-start/introduction/
- [Cargo.toml](quickstart_ecs/Cargo.toml)
```toml
[dependencies]
bevy =  { version = "0.13.2", features = ["dynamic_linking"] }

# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3

[workspace]

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
```
- [main.rs](quickstart_ecs/src/main.rs)
```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        // .add_systems(Startup, add_people)
        // .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}

fn add_people(mut commands: Commands) {...}
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {...}
fn update_people(mut query: Query<&mut Name, With<Person>>) {...}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}
```
- `chain()`
- `Commands.spawn()`
```rust

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);
```

# [three_d_shapes](three_d_shapes/src/main.rs)