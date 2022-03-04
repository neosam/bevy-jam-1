use bevy::prelude::*;

use crate::component;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("UI"))
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        min_size: Size::new(Val::Auto, Val::Px(25.0)),
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(33.3), Val::Auto),
                                ..Default::default()
                            },
                            color: Color::WHITE.into(),
                            ..Default::default()
                        })
                        .insert(component::StrengthUI);
                });
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Px(1.0)),
                    ..Default::default()
                },
                color: Color::WHITE.into(),
                ..Default::default()
            });
        });
}

pub fn update_ui(
    mut ui_query: Query<(&mut UiColor, &mut Style), With<component::StrengthUI>>,
    punch_query: Query<&component::Punch, With<component::Player>>,
) {
    if let (Ok(punch), Ok((mut ui_color, mut style))) =
        (punch_query.get_single(), ui_query.get_single_mut())
    {
        style.size.width = Val::Percent(punch.strength / 3.0 * 100.0);
        if punch.strength >= 3.0 {
            *ui_color = Color::RED.into();
        } else if punch.strength >= 1.0 {
            *ui_color = Color::GREEN.into();
        } else {
            *ui_color = Color::WHITE.into();
        }
    }
}
