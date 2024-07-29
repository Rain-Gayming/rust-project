use bevy::prelude::*;
use bevy_inspector_egui::egui::epaint::color;

mod inventory;
use inventory::*;
use item::*;
use item_type::item_types;

#[derive(Component)] 
struct Player{
    has_been_checked: bool,
    level: i32,
}

fn main() {
    App::new()
        //plugins
        .add_plugins(DefaultPlugins)

        //systems
        .add_systems(Startup, start)
        .add_systems(Update, close_game )
        .add_systems(Update, get_player_data)
        .add_systems(Update, get_item_data)

        .run();
}

fn start(
    mut commands : Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
){
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 12.0, 16.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    );       

    commands.spawn((Player{
        has_been_checked: false,
        level: 1,
    },
        PbrBundle{
            mesh: meshes.add(Cuboid::default()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    ));

    commands.spawn(ItemInfo{
        item_name: "basic sword".to_string()    ,
        item_type: item_types::weapon
    });
}

fn get_item_data(
    mut data: Query<&mut ItemInfo, With<ItemInfo>>
){
    for mut data in data.iter_mut(){
        print!("\n {}'s type is: {:?}", data.item_name, data.item_type);
    }
}

fn get_player_data(
    mut data: Query<&mut Player, With<Player>>
){
    for mut data in data.iter_mut(){
        if data.has_been_checked == false{
            println!("Data found");
            println!("level: {}", data.level);
            data.has_been_checked = true;
        }
    }
}

fn close_game(
    key: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>
){
    if key.just_pressed(KeyCode::Escape){
        exit.send(AppExit::Success);
    }
}           