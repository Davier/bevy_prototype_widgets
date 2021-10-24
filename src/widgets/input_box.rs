use super::Base;
use crate::{Focusable, Widget, WidgetBuilder};
use bevy::{ecs::world::EntityMut, prelude::*, ui::FocusPolicy};

pub struct InputBox {
    base: Base,
    text: Entity,
    caret: Entity,
}

pub struct InputBoxReturnEvent {
    pub source: Entity,
    pub text: String,
}

pub struct InputBoxClearEvent {
    pub target: Entity,
}

pub mod components {
    use bevy::{
        ecs::{
            entity::{EntityMap, MapEntities, MapEntitiesError},
            reflect::ReflectMapEntities,
        },
        prelude::*,
        text::DefaultTextPipeline,
    };

    use crate::Focusable;

    use super::{InputBoxClearEvent, InputBoxReturnEvent};

    #[derive(Reflect, Component)]
    #[reflect(Component, MapEntities)]
    pub struct InputBox {
        pub text: Entity,
        pub caret: Entity,
    }

    // I hate this
    impl FromWorld for InputBox {
        fn from_world(_world: &mut bevy::prelude::World) -> Self {
            Self {
                text: Entity::new(u32::MAX),
                caret: Entity::new(u32::MAX),
            }
        }
    }

    // This could be derived eventually
    impl MapEntities for InputBox {
        fn map_entities(&mut self, entity_map: &EntityMap) -> Result<(), MapEntitiesError> {
            self.text = entity_map.get(self.text)?;
            self.caret = entity_map.get(self.caret)?;
            Ok(())
        }
    }

    #[derive(Reflect, Component)]
    #[reflect(Component, MapEntities)]
    pub struct Caret {
        pub text: Entity,
        pub character_index: usize,
    }

    // I hate this
    impl FromWorld for Caret {
        fn from_world(_world: &mut bevy::prelude::World) -> Self {
            Self {
                text: Entity::new(u32::MAX),
                character_index: 0,
            }
        }
    }

    // This could be derived eventually
    impl MapEntities for Caret {
        fn map_entities(&mut self, entity_map: &EntityMap) -> Result<(), MapEntitiesError> {
            self.text = entity_map.get(self.text)?;
            Ok(())
        }
    }

    pub fn show_caret(
        mut query_caret: Query<&mut Visible, With<Caret>>,
        query_box: Query<(&InputBox, &Focusable), Changed<Focusable>>,
    ) {
        for (input_box, focusable) in query_box.iter() {
            let mut caret_visible = query_caret.get_mut(input_box.caret).unwrap();
            match focusable {
                Focusable::Focused => caret_visible.is_visible = true,
                Focusable::Unfocused => caret_visible.is_visible = false,
            }
        }
    }

    pub fn move_caret(
        mut query_caret: Query<(&Caret, &mut Style), Changed<Caret>>,
        query_text: Query<&Text>,
        text_pipeline: Res<DefaultTextPipeline>,
    ) {
        for (caret, mut style) in query_caret.iter_mut() {
            if caret.character_index == 0 {
                style.position.left = Val::Px(0.);
                style.position.bottom = Val::Px(0.);
            } else if let Some(layout_info) = text_pipeline.get_glyphs(&caret.text) {
                let text = query_text.get(caret.text).unwrap();
                if caret.character_index == text.sections[0].value.len() {
                    style.position.left = Val::Px(layout_info.size.width);
                    style.position.bottom = Val::Px(0.);
                } else {
                    for glyph in &layout_info.glyphs {
                        if glyph.byte_index == caret.character_index {
                            style.position.left =
                                Val::Px(glyph.position.x - glyph.size.x / 2. - 2.);
                            style.position.bottom = Val::Px(0.);
                            break;
                        }
                    }
                }
            }
        }
    }

    const BACKSPACE: char = '\u{8}';
    const DELETE: char = '\u{7f}';
    const RETURN: char = '\r';

    pub fn input_box_keyboard(
        query_box: Query<(Entity, &InputBox, &Focusable)>,
        mut query_text: Query<&mut Text>,
        mut query_caret: Query<&mut Caret>,
        mut character_events: EventReader<ReceivedCharacter>,
        keyboard_input: Res<Input<KeyCode>>,
        mut event_writer: EventWriter<InputBoxReturnEvent>,
    ) {
        // TODO: add marker compoennt to reduce query conflicts?
        for (id, input_box, focusable) in query_box.iter() {
            if matches!(focusable, Focusable::Focused) {
                let mut text = query_text.get_mut(input_box.text).unwrap();
                let string = &mut text.sections[0].value;
                let mut caret = query_caret.get_mut(input_box.caret).unwrap();
                if keyboard_input.just_pressed(KeyCode::Left) {
                    move_left(string, &mut caret.character_index);
                } else if keyboard_input.just_pressed(KeyCode::Right) {
                    move_right(string, &mut caret.character_index);
                }
                for character in character_events.iter() {
                    // TODO: handle WindowId?
                    // TODO: handle multiple text sections
                    match character.char {
                        c if !c.is_control() => {
                            insert_char(string, &mut caret.character_index, c);
                        }
                        BACKSPACE => {
                            move_left(string, &mut caret.character_index);
                            remove_char(string, &mut caret.character_index);
                        }
                        DELETE => remove_char(string, &mut caret.character_index),
                        RETURN => event_writer.send(InputBoxReturnEvent {
                            source: id,
                            text: string.clone(),
                        }),
                        _ => {}
                    }
                }
            }
        }
    }

    fn insert_char(string: &mut String, index: &mut usize, c: char) {
        string.insert(*index, c);
        *index += c.len_utf8();
    }

    fn remove_char(string: &mut String, index: &mut usize) {
        if *index < string.len() {
            string.remove(*index);
        }
    }

    fn move_left(string: &mut String, index: &mut usize) {
        loop {
            if *index == 0 {
                break;
            }
            *index -= 1;
            if string.is_char_boundary(*index) {
                break;
            }
        }
    }

    fn move_right(string: &mut String, index: &mut usize) {
        loop {
            if *index == string.len() {
                break;
            }
            *index += 1;
            if string.is_char_boundary(*index) {
                break;
            }
        }
    }

    pub fn input_box_clear(
        mut event_reader: EventReader<InputBoxClearEvent>,
        query_box: Query<&InputBox>,
        mut query_text: Query<&mut Text>,
        mut query_caret: Query<&mut Caret>,
    ) {
        for event in event_reader.iter() {
            let input_box = query_box.get(event.target).unwrap();
            let mut text = query_text.get_mut(input_box.text).unwrap();
            text.sections[0].value.clear();
            let mut caret = query_caret.get_mut(input_box.caret).unwrap();
            caret.character_index = 0;
        }
    }
}

impl InputBox {
    pub fn new(wb: &WidgetBuilder) -> Self {
        let caret = wb
            .world_mut()
            .spawn()
            .insert_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size {
                        width: Val::Undefined,
                        height: Val::Px(16.),
                    },
                    flex_shrink: 0.,
                    ..Default::default()
                },
                text: Text::with_section(
                    "|",
                    TextStyle {
                        font: wb.default_font.clone(),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            })
            .id();
        let text = wb
            .world_mut()
            .spawn()
            .insert_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexStart,
                    margin: Rect::all(Val::Px(2.0)),
                    size: Size {
                        width: Val::Undefined,
                        height: Val::Px(16.),
                    },
                    flex_shrink: 0.,
                    ..Default::default()
                },
                text: Text::with_section(
                    String::new(),
                    TextStyle {
                        font: wb.default_font.clone(),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            })
            .push_children(&[caret])
            .id();
        wb.world_mut().entity_mut(caret).insert(components::Caret {
            text,
            character_index: 0,
        });
        let base = Base::spawn(wb)
            .insert_bundle(NodeBundle {
                style: Style {
                    margin: Rect::all(Val::Px(2.0)),
                    flex_shrink: 0.,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Interaction::default())
            .insert(FocusPolicy::Block)
            .insert(Focusable::default())
            .insert(components::InputBox { text, caret })
            .push_children(&[text]);
        Self { base, text, caret }
    }

    pub fn text(self, f: impl FnOnce(EntityMut)) -> Self {
        f(self.builder().world_mut().entity_mut(self.text));
        self
    }

    pub fn caret(self, f: impl FnOnce(EntityMut)) -> Self {
        f(self.builder().world.borrow_mut().entity_mut(self.caret));
        self
    }
}

impl Widget for InputBox {
    fn builder(&self) -> &WidgetBuilder {
        self.base.builder()
    }

    fn root_id(&self) -> Entity {
        self.base.root_id()
    }
}
