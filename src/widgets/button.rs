use super::Base;
use crate::{Focusable, Widget, WidgetBuilder};
use bevy::{
    ecs::world::EntityMut,
    prelude::{ButtonBundle, Children, Entity},
    ui::{FocusPolicy, Style},
};

pub struct Button {
    base: Base,
}

pub mod components {
    use bevy::{
        ecs::{entity::MapEntities, reflect::ReflectMapEntities},
        prelude::*,
        sprite::ColorMaterial,
    };

    #[derive(Default, Reflect, Component)]
    #[reflect(Component)]
    pub struct ButtonMaterial {
        pub material: Handle<ColorMaterial>,
        pub material_hovered: Handle<ColorMaterial>,
        pub material_clicked: Handle<ColorMaterial>,
    }

    pub fn update_button_material(
        mut query: Query<
            (&ButtonMaterial, &mut Handle<ColorMaterial>, &Interaction),
            Or<(Changed<ButtonMaterial>, Changed<Interaction>)>,
        >,
    ) {
        for (button, mut material, interaction) in query.iter_mut() {
            *material = match interaction {
                Interaction::Clicked => button.material_clicked.clone(),
                Interaction::Hovered => button.material_hovered.clone(),
                Interaction::None => button.material.clone(),
            };
        }
    }

    #[derive(Reflect, Component, Default)]
    #[reflect(Component, MapEntities)]
    pub struct EventButton<T: Reflect + Default + MapEntities + Clone + Send + Sync + 'static> {
        event: T,
    }

    impl<T: Reflect + Default + MapEntities + Clone + Send + Sync + 'static> EventButton<T> {
        pub fn new(event: T) -> Self {
            Self { event }
        }
    }

    impl<T: Reflect + Default + MapEntities + Clone + Send + Sync + 'static> MapEntities
        for EventButton<T>
    {
        fn map_entities(
            &mut self,
            entity_map: &bevy::ecs::entity::EntityMap,
        ) -> Result<(), bevy::ecs::entity::MapEntitiesError> {
            self.event.map_entities(entity_map)
        }
    }

    pub fn event_button<T: Reflect + Default + MapEntities + Clone + Send + Sync + 'static>(
        query: Query<(&EventButton<T>, &Interaction), Changed<Interaction>>,
        mut event_writer: EventWriter<T>,
    ) {
        for (event_button, interaction) in query.iter() {
            if matches!(interaction, Interaction::Clicked) {
                event_writer.send(event_button.event.clone());
            }
        }
    }
}

impl Button {
    pub fn new(child: impl Widget) -> Self {
        Self::new_impl(child.builder(), child.root_id())
    }

    fn new_impl(wb: &WidgetBuilder, child: Entity) -> Self {
        let base = Base::spawn(wb)
            .insert_bundle(ButtonBundle {
                style: Style {
                    // Stretch width, fix height to child's height
                    // size: Size {
                    //     width: Val::Undefined,
                    //     height: Val::Auto,
                    // },
                    // Auto margins are used to center the text (TextAlignment is not working?)
                    // margin: Rect {
                    //     left: Val::Auto,
                    //     right: Val::Auto,
                    //     top: Val::Px(2.0),
                    //     bottom: Val::Px(2.),
                    // },
                    flex_shrink: 0.,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(components::ButtonMaterial::default())
            .insert(Focusable::default())
            .push_children(&[child]);
        Self { base }.with_child(|mut child| {
            child.insert(FocusPolicy::Pass);
        })
    }

    pub fn with_child(self, f: impl FnOnce(EntityMut)) -> Self {
        let mut world = self.builder().world_mut();
        let child = world.entity(self.root_id()).get::<Children>().unwrap()[0];
        f(world.entity_mut(child));
        drop(world);
        self
    }
}

impl Widget for Button {
    fn builder(&self) -> &WidgetBuilder {
        self.base.builder()
    }

    fn root_id(&self) -> Entity {
        self.base.root_id()
    }
}
