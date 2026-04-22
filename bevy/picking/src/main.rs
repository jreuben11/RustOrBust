//! Demonstrates event bubbling and observer-based picking using bevy's built-in picking system.
//! Picking was upstreamed into bevy in 0.14; bevy_mod_picking is no longer needed.

use bevy::picking::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_message::<DoSomethingComplex>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            receive_greetings.run_if(on_message::<DoSomethingComplex>),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Parent entity: observers here receive bubbled events from children.
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::default(),
        ))
        .observe(change_hue_on_move)
        .observe(
            |trigger: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>| {
                if let Ok(mut transform) = transforms.get_mut(trigger.entity) {
                    transform.rotate_local_y(trigger.delta.x / 50.0);
                }
            },
        )
        .observe(|trigger: On<Pointer<Out>>, time: Res<Time>| {
            info!(
                "[{:.2}]: pointer left {:?}",
                time.elapsed_secs(),
                trigger.entity
            );
        })
        .observe(
            |trigger: On<Pointer<Click>>, mut commands: Commands| {
                if trigger.entity != trigger.observer()
                    && trigger.button == PointerButton::Secondary
                {
                    commands.entity(trigger.entity).despawn();
                }
            },
        )
        .observe(
            |trigger: On<Pointer<Press>>, mut events: MessageWriter<DoSomethingComplex>| {
                events.write(DoSomethingComplex(
                    trigger.entity,
                    trigger.hit.depth,
                ));
            },
        )
        .with_children(|parent| {
            for i in 1..=5 {
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::default())),
                    MeshMaterial3d(materials.add(Color::srgb(1., 0., 0.))),
                    Transform::from_xyz(0.0, 1.0 + 0.5 * i as f32, 0.0)
                        .with_scale(Vec3::splat(0.4)),
                ));
            }
        });

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 4.5, 5.0).looking_at(Vec3::Y * 2.0, Vec3::Y),
    ));
}

fn change_hue_on_move(
    trigger: On<Pointer<Move>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cubes: Query<&MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(MeshMaterial3d(handle)) = cubes.get(trigger.entity) else {
        return;
    };
    let Some(material) = materials.get_mut(handle) else {
        return;
    };
    let hsla = Hsla::from(material.base_color);
    let to_u8 = 255.0 / 360.0;
    let new_hue =
        ((hsla.hue * to_u8) as u8).wrapping_add_signed(trigger.delta.y as i8) as f32
            / to_u8;
    material.base_color = Color::hsla(new_hue, hsla.saturation, hsla.lightness, hsla.alpha);
}

#[derive(Message)]
struct DoSomethingComplex(Entity, f32);

fn receive_greetings(mut greetings: MessageReader<DoSomethingComplex>) {
    for event in greetings.read() {
        info!(
            "Hello {:?}, you are {:?} depth units away from the pointer",
            event.0, event.1
        );
    }
}
