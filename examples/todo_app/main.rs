mod todo_item;
mod todo_list;
use std::fs::File;

use bevy_prototype_widgets::{
    widgets::components::{event_button, EventButton},
    WidgetsPlugin,
};
use todo_item::*;
use todo_list::*;

use bevy::{
    app::{AppExit, Events},
    prelude::*,
    reflect::TypeRegistry,
};

/* TODO:
    - double container (checked/unchecked)
    - separate root to disable ui
    - checkbox
    - radio button
    - tab navigation
    - improve InputBox (move caret to pointer on click, blinking caret, Ctrl+Arrows, Crtl+Backspace, selection)
    - clear input button
    - styling
    - copy/paste support?
    - derive MapEntity?
*/

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 300.,
            height: 400.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WidgetsPlugin)
        .register_type::<DespawnButton>()
        .register_type::<TodoInput>()
        .register_type::<TodoLabel>()
        .register_type::<TodoList>()
        .register_type::<ListAction>()
        .register_type::<EventButton<ListButtonEvent>>()
        .register_type::<ListButtonEvent>()
        .add_event::<ListButtonEvent>()
        .add_system(despawn_button)
        .add_system(input_box_event)
        .add_system(event_button::<ListButtonEvent>)
        .add_system(list_status_event)
        .add_startup_system(setup_ui_camera)
        .add_startup_system(spawn_ui)
        .add_system(
            reload_scene_from_file
                .exclusive_system()
                .with_run_criteria(bevy::ecs::schedule::RunOnce::default()),
        )
        .add_system(save_on_exit.exclusive_system().at_end())
        .run();
}

pub fn setup_ui_camera(mut cmd: Commands) {
    cmd.spawn_bundle(UiCameraBundle::default());
}

pub fn spawn_ui(
    mut cmd: Commands,
    mut scenes: ResMut<Assets<Scene>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    type_registry: Res<TypeRegistry>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Create a template for todo items
    // Templates are useful for parts of the ui that can be spawned dynamically (e.g. items in a list)
    // since it avoids re-assembling them with a `WidgetBuilder` every time
    // TODO: better ergonomics?
    let todo_item_handle = scenes.add(build_todo_item(&type_registry, font.clone(), &mut *colors));
    let todo_item = scenes.get(todo_item_handle.clone()).unwrap();

    // Build the UI in a scene
    let todo_list = build_todo_list(
        &type_registry,
        font,
        &mut *colors,
        todo_item,
        todo_item_handle,
    );
    let todo_list_handle = scenes.add(todo_list);

    // Spawn the scene in the app world
    cmd.spawn_scene(todo_list_handle);
}

// Save the full world on exit
pub fn save_on_exit(world: &mut World) {
    use std::io::Write;
    if !world.get_resource::<Events<AppExit>>().unwrap().is_empty() {
        info!("Saving on exit");
        let type_registry = world.get_resource::<TypeRegistry>().unwrap();
        let scene = DynamicScene::from_world(world, type_registry);
        let mut file = File::create("assets/test.scn.ron").unwrap();
        write!(file, "{}", scene.serialize_ron(type_registry).unwrap()).unwrap();
    }
}

// Load the world on startup if it was saved
// FIXME: loading assets from scenes is not supported yet
//        but this can already be used for hot-reloading of the UI
pub fn reload_scene_from_file(world: &mut World) {
    info!("Loading scene from file");
    // Export to the file if it does't exist yet
    use std::fs::OpenOptions;
    use std::io::Write;

    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        // .create_new(true)
        .create(true)
        .open("assets/test.scn.ron")
    {
        // info!("Scene file did not exist yet, creating it");
        let type_registry = world.get_resource::<TypeRegistry>().unwrap();
        let scene = DynamicScene::from_world(world, type_registry);
        write!(file, "{}", scene.serialize_ron(type_registry).unwrap()).unwrap();
    }
    // Query all the current roots
    let old_roots = world
        .query_filtered::<Entity, Without<Parent>>()
        .iter(world)
        .collect::<Vec<_>>();
    // Re-import the scene while the asset handles are still alive
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let scene_handle: Handle<DynamicScene> = asset_server.load("test.scn.ron");
    asset_server.watch_for_changes().unwrap();
    let mut scene_spawner = world.get_resource_mut::<SceneSpawner>().unwrap();
    scene_spawner.spawn_dynamic(scene_handle);
    // Delete the old roots
    for root in old_roots {
        despawn_with_children_recursive(world, root);
    }
}
