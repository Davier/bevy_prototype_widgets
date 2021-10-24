mod focus;
mod widget;
pub mod widgets;

pub use focus::{CurrentFocus, FocusMaterial, Focusable};
pub use widget::{Widget, WidgetBuilder};

use bevy::{prelude::*, reflect::TypeRegistry};
use std::fs::File;
use std::io::Write;

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<widgets::components::ButtonMaterial>()
            .register_type::<widgets::components::InputBox>()
            .register_type::<widgets::components::Caret>()
            .register_type::<Focusable>()
            .register_type::<FocusMaterial>()
            .add_event::<widgets::InputBoxReturnEvent>()
            .add_event::<widgets::InputBoxClearEvent>()
            .insert_resource(CurrentFocus(None))
            .add_system(widgets::components::update_button_material.system())
            .add_system(widgets::components::show_caret.system())
            .add_system(widgets::components::move_caret.system())
            .add_system(widgets::components::input_box_keyboard.system())
            .add_system(widgets::components::input_box_clear.system())
            .add_system(focus::tab_navigation.system())
            .add_system(focus::focus_material.system())
            .add_system(focus::mouse_focus.system());
    }
}

pub fn print_all(world: &mut World) {
    let type_registry = world.get_resource::<TypeRegistry>().unwrap();

    println!("*****************************************");
    for archetype in world.archetypes().iter() {
        for entity in archetype.entities() {
            println!("{:?}", entity);
            for component_id in archetype.components() {
                let type_info = world.components().get_info(component_id).unwrap();
                let type_id = type_info.type_id().unwrap();
                let type_registry_lock = type_registry.read();
                let registration = type_registry_lock
                    .get(type_id)
                    .unwrap_or_else(|| panic!("{} is not registered", type_info.name()));
                let component = registration
                    .data::<ReflectComponent>()
                    .unwrap_or_else(|| panic!("{} is not a ReflectComponent", type_info.name()))
                    .reflect_component(world, *entity)
                    .unwrap();
                print_reflect(component, None, type_registry, 1);
            }
        }
    }
    // }
}

fn print_reflect(
    reflect: &dyn Reflect,
    name: Option<&str>,
    type_registry: &TypeRegistry,
    depth: u32,
) {
    let type_registry_lock = type_registry.read();
    let type_name = type_registry_lock
        .get(reflect.type_id())
        .map(|registration| registration.short_name())
        .unwrap_or_else(|| reflect.type_name());
    match reflect.reflect_ref() {
        bevy::reflect::ReflectRef::Struct(s) => {
            print_depth(depth);
            if let Some(name) = name {
                print!("{}: ", name);
            }
            println!("{} {{", type_name);
            for (i, field) in s.iter_fields().enumerate() {
                print_reflect(field, s.name_at(i), type_registry, depth + 1);
            }
            print_depth(depth);
            println!("}}");
        }
        bevy::reflect::ReflectRef::TupleStruct(ts) => {
            print_depth(depth);
            if let Some(name) = name {
                print!("{}: ", name);
            }
            println!("{} = (", type_name);
            for (i, field) in ts.iter_fields().enumerate() {
                let name = format!("{}", i);
                print_reflect(field, Some(&name), type_registry, depth + 1);
            }
            print_depth(depth);
            println!(")");
        }
        bevy::reflect::ReflectRef::Tuple(t) => {
            print_depth(depth);
            if let Some(name) = name {
                print!("{}: ", name);
            }
            println!("{} = (", type_name);
            for (i, field) in t.iter_fields().enumerate() {
                let name = format!("{}", i);
                print_reflect(field, Some(&name), type_registry, depth + 1);
            }
            print_depth(depth);
            println!(")");
        }
        bevy::reflect::ReflectRef::List(l) => {
            print_depth(depth);
            if let Some(name) = name {
                print!("{}: ", name);
            }
            println!("{} = [", type_name);
            for field in l.iter() {
                print_reflect(field, None, type_registry, depth + 1);
            }
            print_depth(depth);
            println!("]");
        }
        bevy::reflect::ReflectRef::Map(m) => {
            print_depth(depth);
            if let Some(name) = name {
                print!("{}: ", name);
            }
            println!("{} = [", type_name);
            for (key, value) in m.iter() {
                let key = serialize_reflect(key);
                print_reflect(value, Some(&key), type_registry, depth + 1);
            }
            print_depth(depth);
            println!("]");
        }
        bevy::reflect::ReflectRef::Value(v) => {
            print_depth(depth);
            if let Some(name) = name {
                print!("{}: ", name);
            }
            println!("{} = {},", type_name, serialize_reflect(v));
        }
    }
}

fn print_depth(depth: u32) {
    for _ in 0..depth {
        print!("  ");
    }
}

fn serialize_reflect(reflect: &dyn Reflect) -> String {
    let serializable = reflect.serializable().unwrap();
    let serialize = serializable.borrow();
    ron::to_string(&serialize).unwrap()
}

pub fn export_scene(world: &mut World) {
    let type_registry = world.get_resource::<TypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(world, type_registry);
    let mut file = File::create("assets/test.scn.ron").unwrap();
    write!(file, "{}", scene.serialize_ron(type_registry).unwrap()).unwrap();
}

pub fn import_scene_dynamic(
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    colors.add(Color::rgb(0.2, 0.2, 0.2).into());
    std::mem::forget(font);
    let scene_handle: Handle<DynamicScene> = asset_server.load("test.scn.ron");
    scene_spawner.spawn_dynamic(scene_handle);
    asset_server.watch_for_changes().unwrap();
}

pub fn clear_scene(world: &mut World) {
    let entities = world.query::<Entity>().iter(world).collect::<Vec<_>>();
    for entity in entities {
        world.despawn(entity);
    }
}
