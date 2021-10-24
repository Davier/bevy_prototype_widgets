use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use bevy::{
    ecs::{component::Component, world::EntityMut},
    prelude::*,
    reflect::TypeRegistry,
};

#[derive(Clone)]
pub struct WidgetBuilder {
    pub world: Rc<RefCell<World>>,
    pub default_font: Handle<Font>,
}

impl WidgetBuilder {
    pub fn new(type_registry: &TypeRegistry, default_font: Handle<Font>) -> Self {
        let mut world = World::new();
        world.insert_resource(type_registry.clone());
        Self {
            world: Rc::new(RefCell::new(world)),
            default_font,
        }
    }

    pub fn world(&self) -> Ref<World> {
        self.world.borrow()
    }

    pub fn world_mut(&self) -> RefMut<World> {
        self.world.borrow_mut()
    }
}

impl From<WidgetBuilder> for Scene {
    fn from(wb: WidgetBuilder) -> Self {
        Scene::new(Rc::try_unwrap(wb.world).unwrap().into_inner())
    }
}

pub trait Widget: Sized {
    fn builder(&self) -> &WidgetBuilder;
    fn root_id(&self) -> Entity;
    fn set_root_id(self, entity: &mut Option<Entity>) -> Self {
        *entity = Some(self.root_id());
        self
    }
    // FIXME: this will deadlock
    fn with_root(self, f: impl FnOnce(EntityMut)) -> Self {
        f(self.builder().world_mut().entity_mut(self.root_id()));
        self
    }
    fn get<T: Component, F: Fn(&T)>(self, f: F) -> Self {
        self.with_root(|root| {
            if let Some(component) = root.get::<T>() {
                f(component);
            } else {
                warn!("Component {} does not exist", std::any::type_name::<T>());
            }
        })
    }
    fn get_mut<T: Component, F: FnMut(&mut T)>(self, mut f: F) -> Self {
        self.with_root(|mut root| {
            if let Some(mut component) = root.get_mut::<T>() {
                f(&mut *component);
            } else {
                warn!("Component {} does not exist", std::any::type_name::<T>());
            }
        })
    }
    fn insert<T: Component>(self, component: T) -> Self {
        self.with_root(|mut root| {
            root.insert(component);
        })
    }
    fn insert_bundle<T: Bundle>(self, bundle: T) -> Self {
        self.with_root(|mut root| {
            root.insert_bundle(bundle);
        })
    }
    fn push_children(self, children: &[Entity]) -> Self {
        self.with_root(|mut root| {
            root.push_children(children);
        })
    }
}
