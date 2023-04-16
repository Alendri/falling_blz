mod interaction;
mod math;
mod waves;
mod zones;

use bevy::{
  math::{vec2, vec3},
  prelude::*,
  sprite::MaterialMesh2dBundle,
  window::{PresentMode, WindowResized},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use interaction::hit_check;
use waves::{create_waves, Wave};
use zones::{update_zones, Region, ZoneId, Zones};

pub const TARGET_SIZE: f32 = 30.0;
///Used when checking if any components have left the playing area and should be despawned.
pub const WINDOW_EXPANSION: f32 = 200.0;

#[derive(Debug, Clone)]
pub struct TargetMeta {
  pub angry: bool,
  pub origin_zone: ZoneId,
  pub destination_zone: ZoneId,
}

impl TargetMeta {
  pub fn down() -> Self {
    TargetMeta {
      angry: false,
      origin_zone: ZoneId::Top,
      destination_zone: ZoneId::Bottom,
    }
  }
  pub fn up() -> Self {
    TargetMeta {
      angry: true,
      origin_zone: ZoneId::Bottom,
      destination_zone: ZoneId::Top,
    }
  }
}

#[derive(Debug, Clone)]
pub enum TargetKind {
  Regular(TargetMeta),
  Bubble(TargetMeta),
}
impl TargetKind {
  pub fn is_angry(&self) -> bool {
    match self {
      TargetKind::Bubble(meta) | TargetKind::Regular(meta) => meta.angry,
    }
  }
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
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(setup)
    .add_system(bevy::window::close_on_esc)
    .add_system(apply_velocity)
    .add_system(resize_listening)
    .add_system(spawn)
    .add_system(hit_check)
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
#[derive(Component, Debug)]
pub struct Target(TargetKind);

#[derive(Resource)]
///Keeps track of the current wave index and the wave configuration.
struct Waves {
  waves: Vec<Wave>,
  current_wave: Wave,
  wave_start_time: f64,
  running: bool,
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
      running: true,
    }
  }
}

fn setup(
  mut commands: Commands,
  // mut meshes: ResMut<Assets<Mesh>>,
  // mut materials: ResMut<Assets<ColorMaterial>>,
  windows: Query<&Window>,
) {
  commands.spawn((Camera2dBundle::default(), MainCamera));

  let win = windows
    .get_single()
    .expect("Could not get window reference.");
  let win_w = win.width();
  let win_h = win.height();
  let mut zones = Zones::new();
  update_zones(&mut zones, &win_w, &win_h);
  commands.insert_resource(zones);

  // let m = materials.add(ColorMaterial::from(Color::PURPLE));
}

fn apply_velocity(
  mut commands: Commands,
  time: Res<Time>,
  zones: Res<Zones>,
  mut query: Query<(Entity, &mut Transform, &Velocity, Option<&Target>)>,
) {
  for (entity, mut transform, velocity, target) in query.iter_mut() {
    let x = transform.translation.x + velocity.x * time.delta_seconds();
    let y = transform.translation.y + velocity.y * time.delta_seconds();

    if let Some(target) = target {
      let (destination_zone, angry) = match &target.0 {
        TargetKind::Bubble(meta) | TargetKind::Regular(meta) => {
          (zones.get(&meta.destination_zone), &meta.angry)
        }
      };
      if destination_zone.is_pt_inside(x, y) {
        if !angry {
          //If this a target we should have caught and it is inside its destination call end_game.
          end_game();
        }
        commands.entity(entity).despawn();
      }
    }

    if !zones.get(&ZoneId::WorldBorder).is_pt_inside(x, y) {
      commands.entity(entity).despawn();
    }

    transform.translation.x = x;
    transform.translation.y = y;
  }
}

fn end_game() {
  println!("End game.");
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

  update_zones(zones, width, height);
}

fn spawn(
  mut commands: Commands,
  mut waves: Local<Waves>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  time: Res<Time>,
  zones: Res<Zones>,
) {
  if !waves.running {
    return;
  }
  if waves.current_wave.is_finished(&time.elapsed_seconds_f64()) {
    println!("Finished wave {}.", waves.current_wave.index);
    //Next wave.
    if let Some(new_wave) = waves.waves.pop() {
      waves.current_wave = new_wave;
    } else {
      waves.running = false;
      return;
    }
    waves.wave_start_time = time.elapsed_seconds_f64();
  }

  let wave = &mut waves.current_wave;

  if let Some(bucket) = wave.get_bucket(time.elapsed_seconds_f64()) {
    println!("wave:{:#?}", wave);
    println!("bucket:{:#?}", bucket);
    for i in 0..bucket.count {
      let spawn_zone = zones.get(&bucket.origin);
      let (x, y) = spawn_zone.get_rand_pt();
      let destination_zone = zones.get(&bucket.destination);
      let val = wave.index as isize % 4;
      let mut color = match val {
        1 => Color::LIME_GREEN,
        2 => Color::AQUAMARINE,
        3 => Color::FUCHSIA,
        _ => Color::PURPLE,
      };
      if bucket.kind.is_angry() {
        color = Color::RED
      }
      // println!("spawn:{:?},{:?}   {:?}", x, y, color);
      commands.spawn((
        MaterialMesh2dBundle {
          mesh: meshes.add(shape::Circle::new(TARGET_SIZE).into()).into(),
          material: materials.add(ColorMaterial::from(color)),
          transform: Transform::from_translation(Vec3::new(x as f32, y as f32, i as f32)),
          ..default()
        },
        Target(bucket.kind.clone()),
        Velocity(vec2(0.0, destination_zone.bottom - y as f32).normalize() * wave.velocity),
      ));
    }
  };
}
