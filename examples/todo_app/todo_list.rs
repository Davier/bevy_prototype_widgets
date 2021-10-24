use bevy::{
    ecs::{
        entity::{EntityMap, MapEntities, MapEntitiesError},
        reflect::ReflectMapEntities,
    },
    prelude::*,
    reflect::TypeRegistry,
};
use bevy_prototype_widgets::{
    widgets::{components::EventButton, Button, *},
    *,
};

use crate::todo_item::*;

use serde::{Deserialize, Serialize};

pub fn build_todo_list(
    type_registry: &TypeRegistry,
    font: Handle<Font>,
    colors: &mut Assets<ColorMaterial>,
    todo_item: &Scene,
    todo_item_handle: Handle<Scene>,
) -> Scene {
    // The `WidgetBuilder` assembles `Widget`s into a scene that can be spawned.
    // `Widget`s do not exist in the ECS, they are only functions that spawn entities
    // in the `WidgetBuilder`'s scene.
    let wb = WidgetBuilder::new(type_registry, font);

    // Widgets can be referered to by their entity
    // TODO: find better way than using `Option<Entity>`
    //       but it's not possible to pass an uninitialized variable to `set_root_id()`,
    //       and returning a value would break the flow of the ui tree
    let mut root = None;
    let mut input = None;
    let mut list_checked = None;
    let mut list_unchecked = None;
    let mut hidden_checked = None;
    let mut hidden_unchecked = None;
    let mut button_both = None;
    let mut button_unchecked = None;
    let mut button_checked = None;
    let mut button_clear = None;

    // Main container
    Stack::new_col((
        // Header
        Label::new(&wb, "To-do list")
            // Widgets can have special methods for commonly used properties
            .set_font_size(36.)
            .set_font_color(Color::WHITE),
        // Input
        InputBox::new(&wb)
            // Save its entity to identify it later
            .set_root_id(&mut input),
        // List of unchecked items
        Stack::new_col((
            // Examples are spawned from the item template
            FromScene::new(&wb, todo_item),
            FromScene::new(&wb, todo_item),
            FromScene::new(&wb, todo_item),
            FromScene::new(&wb, todo_item),
            FromScene::new(&wb, todo_item),
            FromScene::new(&wb, todo_item),
        ))
        .set_root_id(&mut list_unchecked)
        // Components are already inserted in the scene's world by the widgets's `new()` functions
        // They can be tweaked like this before spawning the scene in the app world:
        .get_mut(|style: &mut Style| {
            style.flex_grow = 1.;
            style.size.width = Val::Percent(100.);
        })
        .get_mut(|material: &mut Handle<ColorMaterial>| {
            *material = colors.add(Color::rgb(0.2, 0.2, 0.2).into());
        }),
        // List of checked items
        Stack::new_col((FromScene::new(&wb, todo_item),))
            .set_root_id(&mut list_checked)
            .get_mut(|style: &mut Style| {
                style.flex_grow = 0.;
                style.flex_shrink = 0.;
                style.size.height = Val::Undefined;
                style.size.width = Val::Percent(100.);
            })
            .get_mut(|material: &mut Handle<ColorMaterial>| {
                *material = colors.add(Color::rgb(0.1, 0.1, 0.1).into());
            }),
        // Footer (1st row)
        Stack::new_row((
            // Button to show all items
            Button::new(
                Label::new(&wb, "All")
                    .set_font_color(Color::WHITE)
                    .set_font_size(16.),
            )
            .set_root_id(&mut button_both)
            .get_mut(|button_material: &mut components::ButtonMaterial| {
                button_material.material = colors.add(Color::GRAY.into());
                button_material.material_hovered = colors.add(Color::BLUE.into())
            })
            .get_mut(|style: &mut Style| {
                style.flex_grow = 1.;
            }),
            // Button to show only active (unchecked) items
            Button::new(
                Label::new(&wb, "Active")
                    .set_font_color(Color::WHITE)
                    .set_font_size(16.),
            )
            .set_root_id(&mut button_unchecked)
            .get_mut(|button_material: &mut components::ButtonMaterial| {
                button_material.material = colors.add(Color::GRAY.into());
                button_material.material_hovered = colors.add(Color::BLUE.into())
            })
            .get_mut(|style: &mut Style| {
                style.flex_grow = 1.;
            }),
            // Button to show only completed (checked) items
            Button::new(
                Label::new(&wb, "Completed")
                    .set_font_color(Color::WHITE)
                    .set_font_size(16.),
            )
            .set_root_id(&mut button_checked)
            .get_mut(|button_material: &mut components::ButtonMaterial| {
                button_material.material = colors.add(Color::GRAY.into());
                button_material.material_hovered = colors.add(Color::BLUE.into())
            })
            .get_mut(|style: &mut Style| {
                style.flex_grow = 1.;
            }),
        ))
        .get_mut(|style: &mut Style| {
            style.flex_shrink = 0.;
        }),
        // Footer (2nd row)
        Stack::new_row((
            // Print number of unchecked items
            Label::new(&wb, "X items left")
                .set_font_size(16.)
                .get_mut(|style: &mut Style| {
                    style.flex_grow = 1.;
                }),
            // Button to delete all completed items
            Button::new(
                Label::new(&wb, "Clear completed")
                    .set_font_color(Color::WHITE)
                    .set_font_size(16.),
            )
            .set_root_id(&mut button_clear)
            .get_mut(|button_material: &mut components::ButtonMaterial| {
                button_material.material = colors.add(Color::GRAY.into());
                button_material.material_hovered = colors.add(Color::BLUE.into());
            })
            .get_mut(|style: &mut Style| {
                style.flex_grow = 1.;
            }),
        ))
        .get_mut(|style: &mut Style| {
            style.flex_shrink = 0.;
        }),
        // Hidden lists
        Base::spawn(&wb)
            .set_root_id(&mut hidden_checked)
            .insert_bundle(NodeBundle::default())
            .insert(Children::default())
            .get_mut(|style: &mut Style| {
                style.position_type = PositionType::Absolute;
                style.position = Rect {
                    top: Val::Px(10000.),
                    ..Default::default()
                }
            }),
        Base::spawn(&wb)
            .set_root_id(&mut hidden_unchecked)
            .insert_bundle(NodeBundle::default())
            .insert(Children::default())
            .get_mut(|style: &mut Style| {
                style.position_type = PositionType::Absolute;
                style.position = Rect {
                    top: Val::Px(10000.),
                    ..Default::default()
                }
            }),
    ))
    .get_mut(|style: &mut Style| {
        style.size.width = Val::Px(600.);
    })
    .get_mut(|material: &mut Handle<ColorMaterial>| {
        *material = colors.add(Color::BLUE.into());
    })
    .insert(TodoList {
        list_checked: list_checked.unwrap(),
        list_unchecked: list_unchecked.unwrap(),
        hidden_unchecked: hidden_unchecked.unwrap(),
        hidden_checked: hidden_checked.unwrap(),
    })
    .set_root_id(&mut root);

    // Insert a component on the input box that references the container into which
    // items should be inserted
    wb.world_mut().entity_mut(input.unwrap()).insert(TodoInput {
        target: list_unchecked.unwrap(),
        scene: todo_item_handle,
    });

    wb.world_mut()
        .entity_mut(button_both.unwrap())
        .insert(EventButton::new(ListButtonEvent {
            todo_list: root.unwrap(),
            action: ListAction::ShowBoth,
        }));

    wb.world_mut()
        .entity_mut(button_unchecked.unwrap())
        .insert(EventButton::new(ListButtonEvent {
            todo_list: root.unwrap(),
            action: ListAction::ShowUncheckedOnly,
        }));

    wb.world_mut()
        .entity_mut(button_checked.unwrap())
        .insert(EventButton::new(ListButtonEvent {
            todo_list: root.unwrap(),
            action: ListAction::ShowCheckedOnly,
        }));

    wb.world_mut()
        .entity_mut(button_clear.unwrap())
        .insert(EventButton::new(ListButtonEvent {
            todo_list: root.unwrap(),
            action: ListAction::ClearChecked,
        }));

    // Extract the `Scene`
    wb.into()
}

#[derive(Reflect, Component)]
#[reflect(Component, MapEntities)]
pub struct TodoList {
    pub hidden_checked: Entity,
    pub hidden_unchecked: Entity,
    pub list_checked: Entity,
    pub list_unchecked: Entity,
}

// This is a fix for #1395
impl FromWorld for TodoList {
    fn from_world(_world: &mut World) -> Self {
        Self {
            hidden_checked: Entity::new(u32::MAX),
            hidden_unchecked: Entity::new(u32::MAX),
            list_checked: Entity::new(u32::MAX),
            list_unchecked: Entity::new(u32::MAX),
        }
    }
}

// This could be derived eventually
impl MapEntities for TodoList {
    fn map_entities(&mut self, entity_map: &EntityMap) -> Result<(), MapEntitiesError> {
        self.hidden_checked = entity_map.get(self.hidden_checked)?;
        self.hidden_unchecked = entity_map.get(self.hidden_unchecked)?;
        self.list_checked = entity_map.get(self.list_checked)?;
        self.list_unchecked = entity_map.get(self.list_unchecked)?;
        Ok(())
    }
}

#[derive(Reflect, Component)]
#[reflect(Component, MapEntities)]
pub struct TodoInput {
    pub target: Entity,
    pub scene: Handle<Scene>,
}

// This is a fix for #1395
impl FromWorld for TodoInput {
    fn from_world(_world: &mut bevy::prelude::World) -> Self {
        Self {
            target: Entity::new(u32::MAX),
            scene: Default::default(),
        }
    }
}

// This could be derived eventually
impl MapEntities for TodoInput {
    fn map_entities(&mut self, entity_map: &EntityMap) -> Result<(), MapEntitiesError> {
        self.target = entity_map.get(self.target)?;
        Ok(())
    }
}

pub fn input_box_event(
    mut return_event_reader: EventReader<InputBoxReturnEvent>,
    mut clear_event_writer: EventWriter<InputBoxClearEvent>,
    query_input: Query<&TodoInput>,
    mut cmd: Commands,
    mut scenes: ResMut<Assets<Scene>>,
) {
    for event in return_event_reader.iter() {
        if event.text.is_empty() {
            continue;
        }
        if let Ok(input) = query_input.get(event.source) {
            // Spawn a new todo item from the template (`input.scene`) as a child of the target entity
            // FIXME: this only works for one per stage since the scene is spawned when commands are applied
            //        could be fixed by cloning the scene into a new one-shot scene, or giving the command a closure,
            //        or using an exclusive system to spawn instantly
            info!("add item: {}", event.text);
            cmd.entity(input.target).with_children(|parent| {
                // Change the text in the template to the input text
                let world = &mut scenes.get_mut(input.scene.clone()).unwrap().world;
                let mut text = world
                    .query_filtered::<&mut Text, With<TodoLabel>>()
                    .iter_mut(world)
                    .next()
                    .unwrap();
                text.sections[0].value = event.text.clone();
                // Spawn it
                parent.spawn_scene(input.scene.clone());
                // Clear the input box
                clear_event_writer.send(InputBoxClearEvent {
                    target: event.source,
                });
            });
        }
    }
}

#[derive(Clone, Copy, Reflect, Serialize, Deserialize, Debug)]
#[reflect_value(Serialize, Deserialize)]
pub enum ListAction {
    ShowUncheckedOnly,
    ShowCheckedOnly,
    ShowBoth,
    ClearChecked,
}

// FIXME: MapEntities requires Component
#[derive(Clone, Reflect, Debug, Component)]
#[reflect(MapEntities)]
pub struct ListButtonEvent {
    todo_list: Entity,
    action: ListAction,
}

impl Default for ListButtonEvent {
    fn default() -> Self {
        Self {
            todo_list: Entity::new(u32::MAX),
            action: ListAction::ShowBoth,
        }
    }
}

impl MapEntities for ListButtonEvent {
    fn map_entities(&mut self, entity_map: &EntityMap) -> Result<(), MapEntitiesError> {
        self.todo_list = entity_map.get(self.todo_list)?;
        Ok(())
    }
}

pub fn list_status_event(
    mut commands: Commands,
    mut event_reader: EventReader<ListButtonEvent>,
    query_list: Query<&TodoList>,
    mut query_children: Query<&mut Children>,
) {
    for event in event_reader.iter() {
        dbg!(event);
        let todo_list = query_list.get(event.todo_list).unwrap();
        match event.action {
            ListAction::ShowUncheckedOnly => {
                info!("UncheckedOnly");
                // let items = query_children.get_mut(todo_list.list_checked).unwrap();
                // commands
                //     .entity(todo_list.hidden_checked)
                //     .push_children(&items);
                // let items = query_children.get_mut(todo_list.hidden_unchecked).unwrap();
                // commands
                //     .entity(todo_list.list_unchecked)
                //     .push_children(&items);
            }
            ListAction::ShowCheckedOnly => {
                info!("CheckedOnly");
                // let items = query_children.get_mut(todo_list.list_unchecked).unwrap();
                // commands
                //     .entity(todo_list.hidden_unchecked)
                //     .push_children(&items);
                // let items = query_children.get_mut(todo_list.hidden_checked).unwrap();
                // commands
                //     .entity(todo_list.list_checked)
                //     .push_children(&items);
            }
            ListAction::ShowBoth => {
                info!("Both");
                // let items = query_children.get_mut(todo_list.hidden_unchecked).unwrap();
                // commands
                //     .entity(todo_list.list_unchecked)
                //     .push_children(&items);
                // let items = query_children.get_mut(todo_list.hidden_checked).unwrap();
                // commands
                //     .entity(todo_list.list_checked)
                //     .push_children(&items);
            }
            ListAction::ClearChecked => {
                info!("Clear checked");
                // let items = query_children.get_mut(todo_list.list_checked).unwrap();
                // for item in items.iter() {
                //     dbg!(item);
                //     commands.entity(*item).despawn_recursive();
                // }
            }
        }
    }
}
