use bevy::{
    ecs::{
        entity::{EntityMap, MapEntities, MapEntitiesError},
        reflect::ReflectMapEntities,
    },
    prelude::*,
    reflect::TypeRegistry,
};
use bevy_prototype_widgets::{
    widgets::{Button, *},
    *,
};

pub fn build_todo_item(
    type_registry: &TypeRegistry,
    font: Handle<Font>,
    colors: &mut Assets<ColorMaterial>,
) -> Scene {
    let wb = WidgetBuilder::new(type_registry, font);

    let mut button = None;
    let mut row = None;
    Stack::new_row((
        // TODO: checkbox
        // Label of the todo item
        Label::new(&wb, "something")
            .set_font_color(Color::WHITE)
            .set_font_size(16.)
            .get_mut(|style: &mut Style| {
                style.flex_grow = 1.;
                style.margin.left = Val::Px(2.);
            })
            .get_mut(|text: &mut Text| {
                text.alignment.horizontal = HorizontalAlign::Center;
            })
            .insert(TodoLabel::default()),
        // Delete button
        Button::new(Label::new(&wb, "X").set_font_color(Color::WHITE))
            .get_mut(|button_material: &mut components::ButtonMaterial| {
                button_material.material = colors.add(Color::GRAY.into());
                button_material.material_hovered = colors.add(Color::BLUE.into());
            })
            .set_root_id(&mut button),
    ))
    .get_mut(|material: &mut Handle<ColorMaterial>| {
        *material = colors.add(Color::NONE.into());
    })
    .get_mut(|style: &mut Style| {
        style.flex_shrink = 0.;
    })
    .set_root_id(&mut row);

    // Make the delete button despawn the whole row when clicked
    wb.world_mut()
        .entity_mut(button.unwrap())
        .insert(DespawnButton(row.unwrap()));

    // Extract the `Scene`
    wb.into()
}

// Marker component for the label entity of todo items
#[derive(Reflect, Default, Component)]
#[reflect(Component)]
pub struct TodoLabel;

// Component for buttons that despawn the target entity recursively when clicked
#[derive(Reflect, Component)]
#[reflect(Component, MapEntities)]
pub struct DespawnButton(Entity);

// This is a fix for #1395
impl FromWorld for DespawnButton {
    fn from_world(_world: &mut bevy::prelude::World) -> Self {
        Self(Entity::new(u32::MAX))
    }
}

// This could be derived eventually
impl MapEntities for DespawnButton {
    fn map_entities(&mut self, entity_map: &EntityMap) -> Result<(), MapEntitiesError> {
        self.0 = entity_map.get(self.0)?;
        Ok(())
    }
}

pub fn despawn_button(
    mut cmd: Commands,
    query: Query<(&DespawnButton, &Interaction), Changed<Interaction>>,
) {
    for (button, interaction) in query.iter() {
        if interaction == &Interaction::Clicked {
            cmd.entity(button.0).despawn_recursive();
        }
    }
}
