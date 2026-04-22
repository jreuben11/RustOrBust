> **Note (April 2026):** Upgraded to bevy **0.18.1**. `bevy_mod_picking` removed (archived); now uses built-in `bevy::picking`. Events renamed to Messages. Observer API uses `On<E>` instead of `Trigger<E>`.
> Profile/linker settings moved to workspace root `Cargo.toml` / `.cargo/config.toml`.

# quickstart_ecs
- https://bevyengine.org/learn/quick-start/introduction/
- [Cargo.toml](quickstart_ecs/Cargo.toml)
```toml
[dependencies]
bevy = { version = "0.18.1", features = ["dynamic_linking"] }
```
> Dev profile optimizations and mold linker configured at the workspace level in `bevy/Cargo.toml` and `bevy/.cargo/config.toml`.
- [main.rs](quickstart_ecs/src/main.rs)
```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

fn add_people(mut commands: Commands) {...}
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&PersonName, With<Person>>) {...}
fn update_people(mut query: Query<&mut PersonName, With<Person>>) {...}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}
```
- `chain()`; `PersonName` (renamed from `Name` to avoid conflict with `bevy::core::Name` in prelude)
```rust
#[derive(Component)]
struct Person;

#[derive(Component)]
struct PersonName(String);  // 0.18: bevy::core::Name now in prelude

#[derive(Resource)]
struct GreetTimer(Timer);
```

# [three_d_shapes](three_d_shapes/src/main.rs)
- from bevy example

# [pong](pong/src/main.rs)
- from tutorial https://taintedcoders.com/bevy/pong-tutorial/

# [picking](picking/src/main.rs)
- Originally from **bevy_mod_picking** event_listener example (now archived); rewritten for bevy 0.18 built-in picking
- Uses `bevy::picking::prelude::*` + `MeshPickingPlugin`; observers use `On<Pointer<E>>` system param; `Pointer<Down>` → `Pointer<Press>`; messages replace events