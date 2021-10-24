use bevy::prelude::*;
use bevy_prototype_widgets::{import_scene_dynamic, WidgetsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WidgetsPlugin)
        // .add_startup_system(setup_ui_camera.system())
        // .add_startup_system(spawn_all.system())
        // .add_startup_system(export_scene.exclusive_system().at_end())
        .add_startup_system(import_scene_dynamic.exclusive_system().at_start())
        // .add_system(print_all.exclusive_system().at_end())
        .run();
}

pub fn setup_ui_camera(mut cmd: Commands) {
    cmd.spawn_bundle(UiCameraBundle::default());
}

fn _spawn_all(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let stack = cmd
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Stretch,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    cmd.spawn_bundle(TextBundle {
        text: Text::with_section(
            "label 1",
            TextStyle {
                font: font.clone(),
                color: Color::BLACK,
                ..Default::default()
            },
            TextAlignment::default(),
        ),
        style: Style {
            // Stretch width, fix height to text hight
            size: Size {
                width: Val::Undefined,
                height: Val::Px(TextStyle::default().font_size),
            },
            // Auto margins are used to center the text (TextAlignment is not working?)
            margin: Rect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Px(2.0),
                bottom: Val::Px(2.),
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Parent(stack));
    cmd.spawn_bundle(TextBundle {
        text: Text::with_section(
            "label 2",
            TextStyle {
                font,
                color: Color::BLACK,
                ..Default::default()
            },
            TextAlignment::default(),
        ),
        style: Style {
            // Stretch width, fix height to text hight
            size: Size {
                width: Val::Undefined,
                height: Val::Px(TextStyle::default().font_size),
            },
            // Auto margins are used to center the text (TextAlignment is not working?)
            margin: Rect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Px(2.0),
                bottom: Val::Px(2.),
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Parent(stack));
}
