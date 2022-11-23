use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;

const PLAYER_SPEED: f32 = 5.;

fn main() {
    App::new()
        .add_loopless_state(GameStates::AssetsLoading)
        .add_loading_state(
            LoadingState::new(GameStates::AssetsLoading)
                .continue_to_state(GameStates::Main)
                .with_collection::<PlayerAssets>(),
        )
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_enter_system(GameStates::Main, spawn_player)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameStates::Main)
                .with_system(move_player)
                .into(),
        )
        .run();
}

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(path = "images/player/cow.png")]
    idle: Handle<Image>,
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: player_assets.idle.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player);
}

fn move_player(input: Res<Input<KeyCode>>, mut player: Query<&mut Transform, With<Player>>) {
    let mut movement = Vec3::new(0., 0., 0.);
    if input.pressed(KeyCode::W) {
        movement.y += 1.;
    }
    if input.pressed(KeyCode::S) {
        movement.y -= 1.;
    }
    if input.pressed(KeyCode::A) {
        movement.x -= 1.;
    }
    if input.pressed(KeyCode::D) {
        movement.x += 1.;
    }
    if movement == Vec3::ZERO {
        return;
    }
    movement = movement.normalize() * PLAYER_SPEED;
    let mut transform = player.single_mut();
    transform.translation += movement;
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameStates {
    AssetsLoading,
    Main,
}
