use super::Base;
use crate::{Widget, WidgetBuilder};
use bevy::prelude::*;

pub struct Label {
    base: Base,
}

impl Label {
    pub fn new(wb: &WidgetBuilder, label: &str) -> Self {
        Self {
            base: Base::spawn(wb).insert_bundle(TextBundle {
                text: Text::with_section(
                    label,
                    TextStyle {
                        font: wb.default_font.clone(),
                        color: Color::BLACK,
                        ..Default::default()
                    },
                    TextAlignment::default(),
                ),
                style: Style {
                    // Stretch width, fix height to text hight
                    size: Size {
                        width: Val::Undefined,
                        height: Val::Px(TextStyle::default().font_size),
                    },
                    // Auto margins are used to center the text (TextAlignment is not working?)
                    margin: Rect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(2.0),
                        bottom: Val::Px(2.),
                    },
                    flex_shrink: 0.,
                    ..Default::default()
                },
                ..Default::default()
            }),
        }
    }

    pub fn set_font_size(self, font_size: f32) -> Self {
        self.get_mut(|text: &mut Text| {
            text.sections[0].style.font_size = font_size;
        })
        .get_mut(|style: &mut Style| {
            style.min_size.height = Val::Px(font_size);
        })
    }

    pub fn set_font_color(self, color: Color) -> Self {
        self.get_mut(|text: &mut Text| {
            text.sections[0].style.color = color;
        })
    }
}

impl Widget for Label {
    fn builder(&self) -> &WidgetBuilder {
        self.base.builder()
    }

    fn root_id(&self) -> Entity {
        self.base.root_id()
    }
}
