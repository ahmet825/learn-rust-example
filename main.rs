use bevy::prelude::*;

// 1. Küpü kamera ve ışıktan ayırmak için boş bir işaretçi (Marker) bileşeni
#[derive(Component)]
struct DonenKup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 2. Sahne sadece oyun açılırken bir kez kurulmalı (Update yerine Startup)
        .add_systems(Startup, sahne_kur)
        // Döndürme işlemi her karede çalışmalı (Update)
        .add_systems(Update, kubu_dondur)
        .run();
}

fn sahne_kur(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.8))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        // 3. Küpe kimliğini (etiketini) yapıştırıyoruz
        DonenKup,
    ));

    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 3.5, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// 4. Döndürme sistemi
fn kubu_dondur(
    time: Res<Time>,
    mut query:Query<&mut Transform,With<DonenKup>>
){

    for mut transform in &mut query {
        // Y ekseni (dikey eksen) etrafında saniyede 1.5 radyan döndür
        transform.rotate_y(1.5 * time.delta_secs());
    }

}
