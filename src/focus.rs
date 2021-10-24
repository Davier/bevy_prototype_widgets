use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};
use serde::{Deserialize, Serialize};
/*
- UI entities can be Focusable
- at most one entity can be focused, only it is allowed to process keyboard events
- the focused entity should have a visual indicator
- Interaction::Clicked sets focus to that entity
- hitting Tab (Shift+Tab) moves focus to the next (previous) entity: children, or sibling, or parent's sibling, etc. If there is previous focus, focus the 1st entity.
- hitting Esc removes focus
*/

/// Resource
pub struct CurrentFocus(pub Option<Entity>);

#[derive(Reflect, Clone, Copy, Serialize, Deserialize, Component)]
#[reflect_value(Component, Serialize, Deserialize)]
pub enum Focusable {
    Focused,
    Unfocused,
}

impl Default for Focusable {
    fn default() -> Self {
        Self::Unfocused
    }
}

pub fn tab_navigation(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    keyboard_state: Res<Input<KeyCode>>,
    mut current_focus: ResMut<CurrentFocus>,
    mut query_focusable: Query<&mut Focusable>,
) {
    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ElementState::Pressed {
                match key_code {
                    KeyCode::Tab => {
                        if keyboard_state.pressed(KeyCode::LShift) {
                            info!("focus prev");
                        } else {
                            info!("focus next");
                        }
                    }
                    KeyCode::Escape => {
                        info!("unfocus");
                        if let Some(previous_focus) = current_focus.0.take() {
                            *query_focusable.get_mut(previous_focus).unwrap() =
                                Focusable::Unfocused;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Reflect, Default, Component)]
#[reflect(Component)]
pub struct FocusMaterial {
    material: Handle<ColorMaterial>,
    #[reflect(ignore)]
    cache: Option<Handle<ColorMaterial>>,
}

pub fn focus_material(
    mut query: Query<
        (&mut FocusMaterial, &mut Handle<ColorMaterial>, &Focusable),
        Or<(Changed<FocusMaterial>, Changed<Focusable>)>,
    >,
) {
    // TODO: handle Interaction
    for (mut focus_material, mut material, focusable) in query.iter_mut() {
        match focusable {
            Focusable::Focused => {
                focus_material.cache = Some(material.clone());
                *material = focus_material.material.clone();
            }
            Focusable::Unfocused => {
                if let Some(cached) = focus_material.cache.take() {
                    *material = cached;
                }
            }
        }
    }
}

pub fn mouse_focus(
    query_interaction: Query<(Entity, &Interaction), (Changed<Interaction>, With<Focusable>)>,
    mut query_focusable: Query<&mut Focusable>,
    mut current_focus: ResMut<CurrentFocus>,
) {
    for (entity, interaction) in query_interaction.iter() {
        if matches!(interaction, Interaction::Clicked) {
            if let Some(previous_focus) = current_focus.0.take() {
                if let Ok(mut focusable) = query_focusable.get_mut(previous_focus) {
                    *focusable = Focusable::Unfocused;
                }
            }
            info!("mouse focus");
            *query_focusable.get_mut(entity).unwrap() = Focusable::Focused;
            current_focus.0 = Some(entity);
        }
    }
}
