use bevy::prelude::Resource;
use rand::{distributions::Uniform, prelude::Distribution};
use std::ops::Range;

use crate::{TARGET_SIZE, WINDOW_EXPANSION};

#[derive(Resource)]
///Where are the zones.
pub struct Zones {
  pub top: Zone,
  pub bottom: Zone,
  pub expanded_window: Zone,
}

pub struct Zone {
  pub top: f32,
  pub right: f32,
  pub bottom: f32,
  pub left: f32,
  x_between: Uniform<f32>,
  y_between: Uniform<f32>,
}
impl Zone {
  pub fn empty() -> Self {
    Self::new(0.0, 1.0, 1.0, 0.0)
  }
  pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
    let (x_between, y_between) = Zone::create_rng(
      left.min(right)..left.max(right),
      top.min(bottom)..top.max(bottom),
    );
    Zone {
      top,
      right,
      bottom,
      left,
      x_between,
      y_between,
    }
  }
  pub fn update(&mut self, top: f32, right: f32, bottom: f32, left: f32) {
    let (x_between, y_between) = Zone::create_rng(
      left.min(right)..left.max(right),
      top.min(bottom)..top.max(bottom),
    );
    self.top = top;
    self.right = right;
    self.bottom = bottom;
    self.left = left;
    self.x_between = x_between;
    self.y_between = y_between;
  }
}

impl Region for Zone {
  fn get_betweens(&self) -> (&Uniform<f32>, &Uniform<f32>) {
    (&self.x_between, &self.y_between)
  }
  fn get_sides(&self) -> (f32, f32, f32, f32) {
    (self.top, self.right, self.bottom, self.left)
  }
}

pub fn update_zones(zones: &mut Zones, width: &f32, height: &f32) {
  zones.top.update(
    -TARGET_SIZE,
    width - TARGET_SIZE,
    -(height / 8.0),
    TARGET_SIZE,
  );
  zones.bottom.update(
    -height + (height / 8.0),
    width - TARGET_SIZE,
    -height,
    TARGET_SIZE,
  );
  zones.expanded_window.update(
    WINDOW_EXPANSION,
    width + WINDOW_EXPANSION,
    -height - WINDOW_EXPANSION,
    -WINDOW_EXPANSION,
  );
}

pub trait Region {
  fn create_rng(x_range: Range<f32>, y_range: Range<f32>) -> (Uniform<f32>, Uniform<f32>) {
    (
      Uniform::try_from(x_range).expect("Could not create uniform distribution from x_range."),
      Uniform::try_from(y_range).expect("Could not create uniform distribution from y_range."),
    )
  }

  fn get_betweens(&self) -> (&Uniform<f32>, &Uniform<f32>);
  fn get_sides(&self) -> (f32, f32, f32, f32);

  fn get_rand_pt(&self) -> (f32, f32) {
    let (x_between, y_between) = self.get_betweens();
    let mut rng = rand::thread_rng();
    (x_between.sample(&mut rng), y_between.sample(&mut rng))
  }

  fn is_pt_inside(&self, x: f32, y: f32) -> bool {
    let (top, right, bottom, left) = self.get_sides();
    y <= top && y >= bottom && x >= left && x <= right
  }
}
