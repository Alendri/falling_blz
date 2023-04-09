mod regions;
mod waves;
use bevy::{
  math::vec3,
  prelude::*,
  sprite::MaterialMesh2dBundle,
  window::{PresentMode, WindowResized},
};
use regions::{Region, Zone};
use waves::{create_waves, Wave};

const BALL_SIZE: f32 = 30.0;
const INITIAL_BALL_SPEED: f32 = 1.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(1.0, 0.0);

#[derive(Debug)]
pub enum TargetKind {
  Regular,
}

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
    .add_system(spawn)
    .run();
}

//////
//Marker components - Contains no data.
//These are to allow querying specific entities. Such as "All enemies", "The player".
// #[derive(Component)]
// struct Target;
#[derive(Component)]
struct MainCamera;

/////
//Components - Contains some data, structs with data.
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);
#[derive(Component)]
struct Target(TargetKind);

#[derive(Resource)]
///Where are the zones.
struct Zones {
  top: Zone,
  bottom: Zone,
}

#[derive(Resource)]
///Keeps track of the current wave index and the wave configuration.
struct Waves {
  waves: Vec<Wave>,
  current_wave: Wave,
  wave_start_time: f64,
}
impl Default for Waves {
  fn default() -> Self {
    let mut waves = create_waves();
    println!("waves:{:#?}", waves);
    waves.reverse();
    Waves {
      current_wave: waves.pop().unwrap(),
      waves,
      wave_start_time: 0.0,
    }
  }
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  windows: Query<&Window>,
) {
  commands.spawn((Camera2dBundle::default(), MainCamera));

  let win = windows
    .get_single()
    .expect("Could not get window reference.");
  let win_w = win.width();
  let win_h = win.height();
  commands.insert_resource(Zones {
    top: Zone::new(0, win_w as isize, (win_h / 8.0) as isize, 0),
    bottom: Zone::new(
      (-win_h + (win_h / 8.0)) as isize,
      win_w as isize,
      -win_h as isize,
      0,
    ),
  });

  let m = materials.add(ColorMaterial::from(Color::PURPLE));

  // Circle
  commands.spawn((
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(BALL_SIZE).into()).into(),
      material: m.clone(),
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
      ..default()
    },
    Target(TargetKind::Regular),
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
    transform.translation.x += velocity.x * time.delta_seconds();
    transform.translation.y += velocity.y * time.delta_seconds();
  }
}

fn resize_listening(
  mut resize_evts: EventReader<WindowResized>,
  mut cam_q: Query<&mut Transform, With<MainCamera>>,
  mut zones: ResMut<Zones>,
) {
  if resize_evts.is_empty() {
    return;
  }
  println!("Resize handling");
  let mut cam_transform = cam_q.single_mut().into_inner();
  for evt in resize_evts.iter() {
    update_sizing(&mut zones, &mut cam_transform, &evt.width, &evt.height)
  }
}

fn update_sizing(zones: &mut Zones, cam_transform: &mut Transform, width: &f32, height: &f32) {
  //Set the origin to top left of screen instead of center.
  cam_transform.translation = vec3(width / 2.0, -height / 2.0, cam_transform.translation.z);

  zones
    .top
    .update(0, *width as isize, (height / 8.0) as isize, 0);
  zones.bottom.update(
    (-height + (height / 8.0)) as isize,
    *width as isize,
    -height as isize,
    0,
  );
}

fn spawn(
  mut commands: Commands,
  mut waves: Local<Waves>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  time: Res<Time>,
  zones: Res<Zones>,
) {
  if time.elapsed_seconds_f64() - waves.wave_start_time > 10.0 && waves.current_wave.is_finished() {
    //Next wave.
    if let Some(new_wave) = waves.waves.pop() {
      waves.current_wave = new_wave;
    }
    waves.wave_start_time = time.elapsed_seconds_f64();
  }

  let offset = time.elapsed_seconds_f64() - waves.wave_start_time;
  let wave = &mut waves.current_wave;
  if let Some(bucket) = wave.get_bucket(offset) {
    for i in 0..bucket.count {
      let (x, y) = zones.top.get_rand_pt();
      let val = i as isize % 3;
      let color = match val {
        0 => Color::PINK,
        1 => Color::LIME_GREEN,
        2 => Color::AQUAMARINE,
        3 => Color::FUCHSIA,
        _ => Color::PURPLE,
      };
      commands.spawn((
        MaterialMesh2dBundle {
          mesh: meshes.add(shape::Circle::new(BALL_SIZE).into()).into(),
          material: materials.add(ColorMaterial::from(color)),
          transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
          ..default()
        },
        Target(TargetKind::Regular),
        Velocity(INITIAL_BALL_DIRECTION.normalize() * wave.velocity),
      ));
    }
  };
}
