use std::ops::Deref;

use super::Base;
use crate::{Widget, WidgetBuilder};
use bevy::{ecs::world::EntityMut, prelude::*};
use smallvec::{smallvec, SmallVec};

pub struct Stack {
    base: Base,
}

impl Stack {
    pub fn new_col(widgets: impl WidgetTuple) -> Self {
        Stack::new_impl(
            widgets.get_builder(),
            FlexDirection::ColumnReverse,
            &widgets.entities(),
        )
    }

    pub fn new_row(widgets: impl WidgetTuple) -> Self {
        Stack::new_impl(
            widgets.get_builder(),
            FlexDirection::Row,
            &widgets.entities(),
        )
    }

    pub fn new_empty_col(wb: &WidgetBuilder) -> Self {
        Stack::new_impl(wb, FlexDirection::ColumnReverse, &[])
    }

    pub fn new_empty_row(wb: &WidgetBuilder) -> Self {
        Stack::new_impl(wb, FlexDirection::Row, &[])
    }

    pub fn new(
        wb: &WidgetBuilder,
        flex_direction: FlexDirection,
        widgets: impl WidgetTuple,
    ) -> Self {
        Self::new_impl(wb, flex_direction, &widgets.entities())
    }

    fn new_impl(wb: &WidgetBuilder, flex_direction: FlexDirection, children: &[Entity]) -> Self {
        let size = match flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                Size::new(Val::Percent(100.), Val::Auto)
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                Size::new(Val::Auto, Val::Percent(100.))
            }
        };

        Self {
            base: Base::spawn(wb).insert_bundle(NodeBundle {
                style: Style {
                    flex_direction,
                    align_items: AlignItems::Stretch,
                    // align_self: AlignSelf::Center, // FIXME
                    margin: Rect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(0.),
                        bottom: Val::Px(0.),
                    },
                    size,
                    ..Default::default()
                },
                ..Default::default()
            }),
        }
        .push_children(children)
    }

    pub fn with_each_child(self, mut f: impl FnMut(EntityMut)) -> Self {
        let mut world = self.builder().world_mut();
        let children = world
            .entity(self.root_id())
            .get::<Children>()
            .unwrap()
            .clone();
        for child in children.deref() {
            f(world.entity_mut(*child));
        }
        drop(world);
        self
    }
}

impl Widget for Stack {
    fn builder(&self) -> &WidgetBuilder {
        self.base.builder()
    }

    fn root_id(&self) -> Entity {
        self.base.root_id()
    }
}

pub trait WidgetTuple {
    fn entities(&self) -> SmallVec<[Entity; 12]>;
    fn get_builder(&self) -> &WidgetBuilder;
}

macro_rules! impl_widget_tuple {
    {$($index:tt : $name:tt),*} => {

        impl<$($name: Widget),*> WidgetTuple for ($($name,)*) {
            fn entities(&self) -> SmallVec<[Entity; 12]> {
                smallvec![
                    $(self.$index.root_id(),)*
                ]
            }

            fn get_builder(&self) -> &WidgetBuilder {
                let wb = self.0.builder();
                $(assert_eq!(&*self.$index.builder().world() as *const World, &*wb.world() as *const World, "Widgets cannot be composed from different WidgetBuilders");)*
                wb
            }
        }

    }
}

impl_widget_tuple! {0: A}
impl_widget_tuple! {0: A, 1: B}
impl_widget_tuple! {0: A, 1: B, 2: C}
impl_widget_tuple! {0: A, 1: B, 2: C, 3: D}
impl_widget_tuple! {0: A, 1: B, 2: C, 3: D, 4: E}
impl_widget_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F}
impl_widget_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G}
impl_widget_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H}
impl_widget_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I}
impl_widget_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J}
impl_widget_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K}
impl_widget_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L}
