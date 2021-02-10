use bevy::prelude::*;

struct Player;
struct Materials {
    player_material: Handle<ColorMaterial>,
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Game by Burato".to_string(),
            cursor_visible: false,
            vsync: true,
            resizable: false,
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
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(Materials {
            player_material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        });
    let music = asset_server.load("Puppeteer.mp3");
    audio.play(music);
}

fn spawn_player(commands: &mut Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteBundle {
            material: materials.player_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Player);
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Transform, With<Player>>,
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
    }
}
