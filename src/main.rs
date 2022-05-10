use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(open)
        .run();
}

fn open(mut commands: Commands, ass: ResMut<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    let window = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                ..Style::default()
            },
            visibility: Visibility { is_visible: false },
            color: Color::GREEN.into(),
            ..NodeBundle::default()
        })
        .insert(Name::new("Right half window"))
        .id();

    let inventory_window = commands
        .spawn_bundle(NodeBundle {
            color: Color::BLACK.into(),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
            },
            ..NodeBundle::default()
        })
        .id();

    commands.entity(window).push_children(&[inventory_window]);

    let inventory_module = commands
        .spawn_bundle(NodeBundle {
            image: ass.load("Panel_1.png").into(),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..Style::default()
            },
            ..NodeBundle::default()
        })
        .insert(Name::new("Inventory module"))
        .id();

    for _y in 0..5 {
        let row = commands
            .spawn_bundle(NodeBundle {
                visibility: Visibility { is_visible: false },
                ..NodeBundle::default()
            })
            .id();

        for _x in 0..15 {
            let slot = commands
                .spawn_bundle(ImageBundle {
                    color: Color::GRAY.into(),
                    style: Style {
                        size: Size::new(Val::Px(35.0), Val::Px(35.0)),
                        margin: UiRect {
                            left: Val::Px(2.0),
                            right: Val::Px(2.0),
                            top: Val::Px(2.0),
                            bottom: Val::Px(2.0),
                        },
                        ..Style::default()
                    },
                    ..ImageBundle::default()
                })
                .insert(Interaction::default())
                .id();

            commands.entity(row).push_children(&[slot]);
        }

        commands.entity(inventory_module).push_children(&[row]);
    }

    commands
        .entity(inventory_window)
        .insert_children(0, &[inventory_module]);

    let equipment_module = commands
        .spawn_bundle(NodeBundle {
            image: ass.load("Panel_1.png").into(),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..Style::default()
            },
            ..NodeBundle::default()
        })
        .insert(Name::new("Equipment module"))
        .with_children(|module| {
            module
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexEnd,
                        ..Style::default()
                    },
                    visibility: Visibility { is_visible: false },
                    ..NodeBundle::default()
                })
                .insert(Name::new("Row 1"))
                .with_children(|row| {
                    spawn_equipment_slot(row, Size::new(Val::Px(50.0), Val::Px(50.0)), "Head Slot");

                    spawn_equipment_slot(row, Size::new(Val::Px(25.0), Val::Px(25.0)), "Neck Slot");
                });

            module
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexEnd,
                        ..Style::default()
                    },
                    visibility: Visibility { is_visible: false },
                    ..NodeBundle::default()
                })
                .insert(Name::new("Row 1"))
                .with_children(|row| {
                    spawn_equipment_slot(
                        row,
                        Size::new(Val::Px(50.0), Val::Px(100.0)),
                        "Right hand Slot",
                    );

                    spawn_equipment_slot(
                        row,
                        Size::new(Val::Px(50.0), Val::Px(100.0)),
                        "Chest Slot",
                    );

                    spawn_equipment_slot(
                        row,
                        Size::new(Val::Px(50.0), Val::Px(100.0)),
                        "Left hand Slot",
                    );
                });

            module
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexEnd,
                        ..Style::default()
                    },
                    visibility: Visibility { is_visible: false },
                    ..NodeBundle::default()
                })
                .insert(Name::new("Row 1"))
                .with_children(|row| {
                    spawn_equipment_slot(
                        row,
                        Size::new(Val::Px(25.0), Val::Px(25.0)),
                        "Right finger Slot",
                    );

                    spawn_equipment_slot(
                        row,
                        Size::new(Val::Px(50.0), Val::Px(25.0)),
                        "Waist Slot",
                    );

                    spawn_equipment_slot(
                        row,
                        Size::new(Val::Px(25.0), Val::Px(25.0)),
                        "Left finger Slot",
                    );
                });

            module
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexEnd,
                        ..Style::default()
                    },
                    visibility: Visibility { is_visible: false },
                    ..NodeBundle::default()
                })
                .insert(Name::new("Row 1"))
                .with_children(|row| {
                    spawn_equipment_slot(
                        row,
                        Size::new(Val::Px(50.0), Val::Px(50.0)),
                        "Hands Slot",
                    );

                    spawn_equipment_slot(
                        row,
                        Size::new(Val::Px(50.0), Val::Px(100.0)),
                        "Legs Slot",
                    );

                    spawn_equipment_slot(row, Size::new(Val::Px(50.0), Val::Px(50.0)), "Feet Slot");
                });
        })
        .id();

    commands
        .entity(inventory_window)
        .push_children(&[equipment_module]);
}

fn spawn_equipment_slot(parent: &mut ChildBuilder, size: Size<Val>, name: &'static str) -> Entity {
    parent
        .spawn_bundle(ImageBundle {
            color: Color::GRAY.into(),
            style: Style {
                size,
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                },
                ..Style::default()
            },
            ..ImageBundle::default()
        })
        .insert(Name::new(name))
        .insert(Interaction::default())
        .id()
}
