use crate::{Widget, WidgetBuilder};
use bevy::prelude::*;

pub struct Base {
    wb: WidgetBuilder,
    root: Entity,
}

impl Base {
    pub fn spawn(wb: &WidgetBuilder) -> Self {
        let wb = wb.clone();
        let root = wb.world_mut().spawn().id();
        Self { wb, root }
    }
}

impl Widget for Base {
    fn builder(&self) -> &WidgetBuilder {
        &self.wb
    }

    fn root_id(&self) -> Entity {
        self.root
    }
}
