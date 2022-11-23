use bevy::{
    prelude::*,
    utils::HashMap,
};
use bevy_asset_loader::prelude::*;
use bevy_match3::prelude::*;
use iyes_loopless::prelude::*;

const GEM_LENGTH: f32 = 50.;

fn main() {
    App::new()
        .add_loopless_state(GameStates::AssetsLoading)
        .add_loading_state(
            LoadingState::new(GameStates::AssetsLoading)
                .continue_to_state(GameStates::Main)
                .with_collection::<GemsAssets>(),
        )
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    resizable: false,
                    title: "Gems".to_string(),
                    ..WindowDescriptor::default()
                },
                ..default()
            })
        )
        .add_plugin(Match3Plugin)
        .add_enter_system(GameStates::Main, spawn_gems)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameStates::Main)
                .with_system(move_player)
                .into(),
        )
        .run();
}

#[derive(AssetCollection, Resource)]
struct GemsAssets {
    #[asset(path = "images/gems/blue.png")]
    blue: Handle<Image>,
    #[asset(path = "images/gems/green.png")]
    green: Handle<Image>,
    #[asset(path = "images/gems/red.png")]
    red: Handle<Image>,
}

#[derive(Component, Clone)]
struct VisibleBoard(HashMap<UVec2, Entity>);
#[derive(Component)]
struct MainCamera;

fn spawn_gems(mut commands: Commands, board: Res<Board>, gems_assets: Res<GemsAssets>) {
    let board_length = GEM_LENGTH * 10.0;
    let center_offset_x = board_length / 2.0 - GEM_LENGTH / 2.0;
    let center_offset_y = board_length / 2.0 - GEM_LENGTH / 2.0;

    let mut camera = Camera2dBundle::default();
    camera.transform = Transform::from_xyz(
        center_offset_x, 
        0.0 - center_offset_y, 
        camera.transform.translation.z,
    );
    commands.spawn(camera).insert(MainCamera);

    let mut gems = HashMap::default();
    let vis_board = commands.spawn(SpatialBundle::default()).id();

    board.iter().for_each(|(position, typ)| {
        let transform = Transform::from_xyz(
            position.x as f32 * GEM_LENGTH, 
            position.y as f32 * -GEM_LENGTH, 
            0.0,
        );
        let gem_texture = match typ % 3 {
            0 => gems_assets.blue.clone(),
            1 => gems_assets.green.clone(),
            2 => gems_assets.red.clone(),
            _ => gems_assets.blue.clone(),
        };

        let child = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(GEM_LENGTH, GEM_LENGTH)),
                ..Default::default()
            },
            transform,
            texture: gem_texture,
            ..Default::default()
        })
        .insert(Name::new(format!("{};{}", position.x, position.y)))
        .id();

        gems.insert(*position, child);
        commands.entity(vis_board).add_child(child);
    });

    let board = VisibleBoard(gems);
    commands.entity(vis_board).insert(board);
}


fn move_player(input: Res<Input<KeyCode>>, mut player: Query<&mut Transform, With<VisibleBoard>>) {
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
    movement = movement.normalize() * 5.;
    let mut transform = player.single_mut();
    transform.translation += movement;
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameStates {
    AssetsLoading,
    Main,
}
