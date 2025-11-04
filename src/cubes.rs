use bevy::prelude::*;
use rand::prelude::*;

pub fn cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let count = 100000;
    let size = (count as f32).cbrt().round();
    let gap = 1.25;
    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(1.0)));
    let material = materials.add(StandardMaterial::from(Color::WHITE));

    let cubes = (0..count).map(move |i| {
        let x = i as f32 % size;
        let y = (i as f32 / size) % size;
        let z = i as f32 / (size * size);
        let num = rand::rng().random_range(1.0..15.0);
        let pos = Vec3::new(x * gap, y * gap, z * gap);
        (
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Position(pos),
            Angle(num),
        )
    });

    commands.spawn_batch(cubes);
}

pub fn position_cubes(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation = position.0;
    }
}
pub fn rotate_cubes(time: Res<Time>, mut query: Query<(&mut Transform, &Angle)>) {
    for (mut transform, angle) in query.iter_mut() {
        transform.rotate_y(angle.0 * time.delta_secs());
    }
}

#[derive(Component)]
pub struct Position(Vec3);

#[derive(Component)]
pub struct Angle(f32);
