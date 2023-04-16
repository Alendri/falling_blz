use std::f32::consts::PI;

use bevy::{math::vec2, prelude::Vec2};

pub const HALF_PI: f32 = PI / 2.0;

pub fn angle_to_unit_vec(angle: f32) -> Vec2 {
  vec2(angle.cos(), angle.sin())
}

#[cfg(test)]
mod tests {
  use std::f32::consts::PI;

  use float_eq::assert_float_eq;

  use super::*;

  #[test]
  fn down() {
    let result = angle_to_unit_vec(-HALF_PI);
    assert_float_eq!(
      (result.x, result.y),
      (0.0, -1.0),
      abs <= (0.000_000_1, 0.000_000_1)
    );
  }
  #[test]
  fn up() {
    let result = angle_to_unit_vec(HALF_PI);
    assert_float_eq!(
      (result.x, result.y),
      (0.0, 1.0),
      abs <= (0.000_000_1, 0.000_000_1)
    );
  }
  #[test]
  fn right() {
    let result = angle_to_unit_vec(0.0);
    assert_float_eq!(
      (result.x, result.y),
      (1.0, 0.0),
      abs <= (0.000_000_1, 0.000_000_1)
    );
  }
  #[test]
  fn left() {
    let result = angle_to_unit_vec(PI);
    assert_float_eq!(
      (result.x, result.y),
      (-1.0, 0.0),
      abs <= (0.000_000_1, 0.000_000_1)
    );
  }
}
