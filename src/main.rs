use bevy::prelude::*;

struct Player;
struct CameraMain;
struct Npc;
struct Materials {
    player_material: Handle<ColorMaterial>,
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Game by Burato".to_string(),
            cursor_visible: true,
            vsync: false,
            resizable: true,
            //mode: WindowMode::Fullscreen { use_size: false },
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_player.system()))
        .add_system(player_movement.system())
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle {
            ..Default::default()
        })
        .with(CameraMain)
        .insert_resource(Materials {
            player_material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        });
}

fn spawn_player(commands: &mut Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteBundle {
            material: materials.player_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            transform: Transform::from_translation(Vec3::new(200.,200.,0.0)),
            ..Default::default()
        })
        .with(Npc);

        commands
        .spawn(SpriteBundle {
            material: materials.player_material.clone(),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .with(Player);

}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Transform, With<Player>>,
    mut camera_positions: Query<&mut Transform, With<CameraMain>>,
) {
    for mut transform in player_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 2.;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::E) {
            transform.rotate(Quat::from_rotation_z(- 0.1));
        }
        if keyboard_input.pressed(KeyCode::Q) {
            transform.rotate(Quat::from_rotation_z(0.1));
        }
        if keyboard_input.pressed(KeyCode::R) {
            transform.rotate(Quat::from_rotation_x(0.1));
        }
    }
    for mut transform in camera_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 2.;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::E) {
            transform.rotate(Quat::from_rotation_z(- 0.1));
        }
    }
}
