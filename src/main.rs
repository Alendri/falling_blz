use bevy::{
  math::vec3,
  prelude::*,
  sprite::MaterialMesh2dBundle,
  window::{PresentMode, WindowResized},
};

const BALL_SIZE: f32 = 30.0;
const INITIAL_BALL_SPEED: f32 = 1.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(1.0, 0.0);

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        present_mode: PresentMode::AutoVsync,
        ..default()
      }),
      ..default()
    }))
    .add_startup_system(setup)
    .add_system(bevy::window::close_on_esc)
    .add_system(apply_velocity)
    .add_system(resize_listening)
    .run();
}

//////
//Marker components - Contains no data.
//These are to allow querying specific entities. Such as "All enemies", "The player".
#[derive(Component)]
struct Ball;
#[derive(Component)]
struct MainCamera;

/////
//Components - Contains some data, structs with data.
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Resource)]
///How far down the bottom zone starts and finishes.
struct BottomZone {
  start: isize,
  middle: isize,
  bottom: isize,
}
#[derive(Resource)]
///How far down the top zone stretches.
struct TopZone {
  start: isize,
  middle: isize,
  bottom: isize,
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn((Camera2dBundle::default(), MainCamera));

  commands.insert_resource(BottomZone {
    bottom: 0,
    middle: 0,
    start: 0,
  });
  commands.insert_resource(TopZone {
    bottom: 80,
    middle: 40,
    start: 20,
  });

  // Circle
  commands.spawn((
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(BALL_SIZE).into()).into(),
      material: materials.add(ColorMaterial::from(Color::PURPLE)),
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
      ..default()
    },
    Ball,
    Velocity(INITIAL_BALL_DIRECTION.normalize() * INITIAL_BALL_SPEED),
  ));

  // Hexagon
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes
      .add(shape::RegularPolygon::new(50.0, 6).into())
      .into(),
    material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
    transform: Transform::from_translation(Vec3::new(150.0, 0.0, 0.0)),
    ..default()
  });
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
  for (mut transform, velocity) in &mut query {
    // println!(
    //   "translation x{} * delta{} = {}",
    //   velocity.x,
    //   time.delta_seconds(),
    //   time.delta_seconds() * velocity.x
    // );
    transform.translation.x += velocity.x * time.delta_seconds();
    transform.translation.y += velocity.y * time.delta_seconds();
  }
}

fn resize_listening(
  mut resize_evts: EventReader<WindowResized>,
  mut query: Query<&mut Transform, With<MainCamera>>,
  mut bz: ResMut<BottomZone>,
) {
  if resize_evts.is_empty() {
    return;
  }
  println!("Resize handling");
  let mut cam_transform = query.single_mut().into_inner();
  for evt in resize_evts.iter() {
    // cam_transform.translation = vec3(
    //   evt.width / 2.0,
    //   -evt.height / 2.0,
    //   cam_transform.translation.z,
    // );
    update_sizing(&mut bz, &mut cam_transform, &evt.width, &evt.height)
  }
}

fn update_sizing(bz: &mut BottomZone, cam_transform: &mut Transform, width: &f32, height: &f32) {
  //Set the origin to top left of screen instead of center.
  cam_transform.translation = vec3(width / 2.0, -height / 2.0, cam_transform.translation.z);

  bz.start = -height as isize + 100;
  bz.middle = -height as isize + 50;
  bz.bottom = -height as isize + 20;
}
