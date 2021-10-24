use crate::{Widget, WidgetBuilder};
use bevy::{
    ecs::{entity::EntityMap, reflect::ReflectMapEntities},
    prelude::*,
    reflect::TypeRegistry,
};

pub struct FromScene {
    wb: WidgetBuilder,
    root: Entity,
}

impl FromScene {
    pub fn new(wb: &WidgetBuilder, scene: &Scene) -> Self {
        let root = spawn_scene(&mut *wb.world_mut(), scene);
        Self {
            wb: wb.clone(),
            root,
        }
    }
}

impl Widget for FromScene {
    fn builder(&self) -> &WidgetBuilder {
        &self.wb
    }

    fn root_id(&self) -> Entity {
        self.root
    }
}

fn spawn_scene(world: &mut World, scene: &Scene) -> Entity {
    let type_registry = world.get_resource::<TypeRegistry>().unwrap().clone();
    let type_registry = type_registry.read();
    let mut entity_map = EntityMap::default();

    for archetype in scene.world.archetypes().iter() {
        for scene_entity in archetype.entities() {
            let entity = *entity_map
                .entry(*scene_entity)
                .or_insert_with(|| world.spawn().id());
            for component_id in archetype.components() {
                let component_info = scene
                    .world
                    .components()
                    .get_info(component_id)
                    .expect("component_ids in archetypes should have ComponentInfo");

                let registration = type_registry
                    .get(component_info.type_id().unwrap())
                    .unwrap_or_else(|| panic!("Unregistered type: {}", component_info.name()));
                let reflect_component = registration
                    .data::<ReflectComponent>()
                    .unwrap_or_else(|| panic!("Unregistered component: {}", component_info.name()));
                reflect_component.copy_component(&scene.world, world, *scene_entity, entity);
            }
        }
    }
    for registration in type_registry.iter() {
        if let Some(map_entities_reflect) = registration.data::<ReflectMapEntities>() {
            map_entities_reflect
                .map_entities(world, &entity_map)
                .unwrap();
        }
    }
    for entity_id in entity_map.values() {
        let entity = world.entity_mut(entity_id);
        if !entity.contains::<Parent>() {
            return entity_id;
        }
    }
    panic!("Empty scene");
}
