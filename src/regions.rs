use rand::{distributions::Uniform, prelude::Distribution};
use std::ops::Range;

pub struct Zone {
  top: f32,
  right: f32,
  bottom: f32,
  left: f32,
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
}

pub trait Region {
  fn create_rng(x_range: Range<f32>, y_range: Range<f32>) -> (Uniform<f32>, Uniform<f32>) {
    (
      Uniform::try_from(x_range).expect("Could not create uniform distribution from x_range."),
      Uniform::try_from(y_range).expect("Could not create uniform distribution from y_range."),
    )
  }

  fn get_betweens(&self) -> (&Uniform<f32>, &Uniform<f32>);

  fn get_rand_pt(&self) -> (f32, f32) {
    let (x_between, y_between) = self.get_betweens();
    let mut rng = rand::thread_rng();
    (x_between.sample(&mut rng), y_between.sample(&mut rng))
  }
}
