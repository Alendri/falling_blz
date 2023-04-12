use bevy::{math::vec3, prelude::*};

use crate::{Target, TARGET_SIZE};

pub fn hit_check(
  mut commands: Commands,
  windows: Query<&Window>,
  query: Query<(Entity, &Transform), With<Target>>,
) {
  let win = windows
    .get_single()
    .expect("Could not get window reference.");

  if let Some(cursor_position) = win.cursor_position() {
    //Pos has origin bottom left, camera has top left, adjust!
    let cursor_y = -win.height() + cursor_position.y;

    let m_pos = vec3(cursor_position.x, cursor_y, 0.0);

    for (entity, transform) in query.iter() {
      if transform.translation.distance(m_pos) < TARGET_SIZE {
        commands.entity(entity).despawn();
      }
    }
  } else {
    return;
  }
}
