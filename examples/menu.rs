use std::time::Duration;

use bevy::{color::palettes, prelude::*, render::texture::TextureFormatPixelInfo};

use bevy_easings::*;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_logo_points, switch_menu, update_text))
        .add_systems(PostUpdate, despawn_menu)
        .run();

    Ok(())
}

fn switch_menu(
    mut commands: Commands,
    menu: Query<(Entity, &MenuItem, Option<&ToDespawn>)>,
    mut timer: Local<Option<Timer>>,
    time: Res<Time>,
) {
    if !timer
        .as_mut()
        .map(|timer| timer.tick(time.delta()).just_finished())
        .unwrap_or(true)
    {
        return;
    }
    if menu.is_empty() {
        spawn_menu(&mut commands);
        *timer = Some(Timer::from_seconds(5.0, TimerMode::Once));
    } else {
        for (entity, item, _) in menu.iter() {
            match item {
                MenuItem::Root => {
                    commands.entity(entity).insert((
                        Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            top: Val::Percent(0.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        }
                        .ease_to(
                            Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                top: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            EaseFunction::QuadraticIn,
                            EasingType::Once {
                                duration: Duration::from_secs(1),
                            },
                        )
                        .delay(Duration::from_secs_f32(0.8)),
                        ToDespawn,
                    ));
                }
                MenuItem::Button(i) => {
                    commands.entity(entity).insert(
                        Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            top: Val::Px(30.0 + *i as f32 * 70.0),
                            border: UiRect::all(Val::Px(0.0)),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        }
                        .ease_to(
                            Style {
                                width: Val::Px(0.0),
                                height: Val::Px(0.0),
                                border: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            EaseFunction::QuadraticOut,
                            EasingType::Once {
                                duration: Duration::from_secs_f32(1.2),
                            },
                        )
                        .delay(Duration::from_secs_f32(0.2 * (5 - i) as f32)),
                    );
                }
                _ => (),
            }
        }
        *timer = Some(Timer::from_seconds(3.0, TimerMode::Once));
    }
}

#[derive(Component)]
struct ToDespawn;

fn despawn_menu(
    mut commands: Commands,
    menu: Query<&MenuItem, With<ToDespawn>>,
    mut finished_easing: RemovedComponents<EasingComponent<Style>>,
) {
    for finished in finished_easing.read() {
        if menu.contains(finished) {
            commands.entity(finished).despawn_recursive();
        }
    }
}

#[derive(Component)]
enum MenuItem {
    Root,
    Panel,
    Button(u32),
}

#[derive(Resource)]
struct Logo(Handle<Image>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Logo(asset_server.load("vleue.png")));

    commands.spawn(Camera2dBundle::default());
}

fn spawn_menu(commands: &mut Commands) {
    let border_diff = 3.0;

    commands
        .spawn((
            NodeBundle { ..default() },
            Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                top: Val::Percent(-100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }
            .ease_to(
                Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    top: Val::Percent(0.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                EaseFunction::QuadraticOut,
                EasingType::Once {
                    duration: Duration::from_secs(1),
                },
            ),
            MenuItem::Root,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        background_color: palettes::tailwind::EMERALD_400.into(),
                        border_radius: BorderRadius::all(Val::Percent(5.0)),
                        border_color: BorderColor(palettes::tailwind::EMERALD_100.into()),
                        z_index: ZIndex::Global(1),
                        ..default()
                    },
                    Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(5.0)),
                        width: Val::Px(500.0),
                        height: Val::Px(400.0),
                        ..default()
                    }
                    .ease_to(
                        Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(5.0 + border_diff)),
                            width: Val::Px(500.0 + border_diff * 2.0),
                            height: Val::Px(400.0 + border_diff * 2.0),
                            ..default()
                        },
                        EaseFunction::QuadraticInOut,
                        EasingType::PingPong {
                            duration: Duration::from_secs_f32(0.2),
                            pause: None,
                        },
                    ),
                    MenuItem::Panel,
                ))
                .with_children(|parent| {
                    for i in 0..5 {
                        parent
                            .spawn((
                                ButtonBundle {
                                    image: UiImage::default()
                                        .with_color(palettes::tailwind::INDIGO_800.into()),
                                    border_radius: BorderRadius::all(Val::Percent(10.0)),
                                    border_color: BorderColor(
                                        palettes::tailwind::INDIGO_400.into(),
                                    ),
                                    style: Style {
                                        width: Val::Px(0.0),
                                        height: Val::Px(0.0),
                                        top: Val::Px(30.0 + i as f32 * 70.0),
                                        border: UiRect::all(Val::Px(0.0)),
                                        position_type: PositionType::Absolute,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                Style {
                                    width: Val::Px(0.0),
                                    height: Val::Px(0.0),
                                    top: Val::Px(30.0 + i as f32 * 70.0),
                                    border: UiRect::all(Val::Px(0.0)),
                                    position_type: PositionType::Absolute,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                }
                                .ease_to(
                                    Style {
                                        width: Val::Px(250.0),
                                        height: Val::Px(65.0),
                                        border: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    },
                                    EaseFunction::BounceOut,
                                    EasingType::Once {
                                        duration: Duration::from_secs_f32(1.2),
                                    },
                                )
                                .delay(Duration::from_secs_f32(0.2 * i as f32)),
                                MenuItem::Button(i),
                            ))
                            .with_children(|p| {
                                p.spawn(TextBundle {
                                    text: Text::from_section(
                                        match i {
                                            0 => "New Game",
                                            1 => "Load Game",
                                            2 => "Options",
                                            3 => "Credits",
                                            4 => "Quit",
                                            _ => unreachable!(),
                                        },
                                        TextStyle {
                                            font_size: 0.0,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                });
                            });
                    }
                });
        });
}

fn spawn_logo_points(
    logo: Res<Logo>,
    images: Res<Assets<Image>>,
    window: Query<&Window>,
    mut commands: Commands,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    let Some(image) = images.get(&logo.0) else {
        return;
    };

    let resolution = 6;
    let window_size = window.single().physical_size().as_vec2();

    for i in (0..image.width()).step_by(resolution) {
        for j in (0..image.height()).step_by(resolution) {
            let pixel_size = image.texture_descriptor.format.pixel_size();
            let value = image
                .data
                .chunks(pixel_size)
                .nth((j * image.width() + i) as usize)
                .unwrap();
            // ignore transparent pixels
            if value[3] == 0 {
                continue;
            }
            commands.spawn((
                NodeBundle {
                    z_index: ZIndex::Global(0),
                    border_radius: BorderRadius::MAX,
                    ..Default::default()
                },
                Style {
                    width: Val::Px(resolution as f32),
                    height: Val::Px(resolution as f32),
                    left: Val::Px(rand::thread_rng().gen_range(0.0..window_size.x)),
                    top: Val::Px(rand::thread_rng().gen_range(0.0..window_size.y)),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                }
                .ease_to(
                    Style {
                        width: Val::Px(resolution as f32),
                        height: Val::Px(resolution as f32),
                        left: Val::Px(i as f32 + 10.0),
                        top: Val::Px(j as f32 + window_size.y / 2.0 - image.height() as f32 / 2.0),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    EaseFunction::QuadraticInOut,
                    EasingType::PingPong {
                        duration: std::time::Duration::from_secs_f32(2.5),
                        pause: Some(std::time::Duration::from_secs(1)),
                    },
                ),
                BackgroundColor(Color::Oklaba(Oklaba::new(
                    0.5,
                    rand::thread_rng().gen_range(-1.5..1.5),
                    rand::thread_rng().gen_range(-1.5..1.5),
                    1.0,
                )))
                .ease_to(
                    BackgroundColor(Color::Srgba(Srgba::new(
                        value[0] as f32 / u8::MAX as f32,
                        value[1] as f32 / u8::MAX as f32,
                        value[2] as f32 / u8::MAX as f32,
                        1.0,
                    ))),
                    EaseFunction::QuadraticOut,
                    EasingType::PingPong {
                        duration: std::time::Duration::from_secs_f32(2.5),
                        pause: Some(std::time::Duration::from_secs(1)),
                    },
                ),
            ));
        }
    }

    *done = true;
}

// Trick for now as Bevy doesn't support dynamic font size
fn update_text(mut text: Query<(&mut Text, &Parent)>, nodes: Query<&Node>) {
    for (mut text, parent) in text.iter_mut() {
        let node = nodes.get(parent.get()).unwrap();
        text.sections[0].style.font_size = (node.size().y / 4.0).floor() * 2.0;
    }
}
