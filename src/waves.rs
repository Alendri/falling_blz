use crate::TargetKind;
use bevy::render::render_resource::encase::rts_array::Length;
use rand::Rng;

static DEFAULT_WAVE_DURATION: f64 = 10.0;

#[derive(Debug)]
pub struct WaveBucket {
  pub kind: TargetKind,
  pub count: isize,
  spawn_offset: f64,
}

#[derive(Debug)]
///Data for the contents of a single wave.
pub struct Wave {
  pub index: usize,
  pub velocity: f32,
  pub buckets: Vec<WaveBucket>,
  start_time: Option<f64>,
  duration: f64,
}

impl Wave {
  pub fn is_finished(&self, time: &f64) -> bool {
    // println!("{}, self.start_time:{:?}", self.index, self.start_time);

    if let Some(start_time) = self.start_time {
      self.buckets.len() == 0 && time - start_time >= self.duration
    } else {
      false
    }
  }
  pub fn get_bucket(&mut self, time: f64) -> Option<WaveBucket> {
    if self.buckets.len() == 0 {
      return None;
    }
    if self.start_time.is_none() {
      self.start_time = Some(time);
      return self.buckets.pop();
    }
    let start = self.start_time.unwrap();
    let current_offset = time - start;
    if current_offset >= self.buckets.last().unwrap().spawn_offset {
      return self.buckets.pop();
    }
    None
  }
}

#[derive(Debug)]
struct BuilderWave {
  buckets: Vec<(TargetKind, isize)>,
  velocity: f32,
  duration: f64,
}

#[derive(Default, Debug)]
pub struct WavesBuilder {
  waves: Vec<BuilderWave>,
  velocity: f32,
  velocity_increase: f32,
  duration: f64,
}

impl WavesBuilder {
  pub fn new(velocity: f32) -> WavesBuilder {
    WavesBuilder {
      velocity,
      duration: DEFAULT_WAVE_DURATION,
      ..Default::default()
    }
  }

  pub fn add(self, kind: TargetKind, count: isize) -> WavesBuilder {
    let mut builder = if self.waves.is_empty() {
      self.wave()
    } else {
      self
    };

    let wave = builder.waves.last_mut().unwrap();
    wave.buckets.push((kind, count));

    builder
  }

  ///Add a new wave.
  pub fn wave(mut self) -> WavesBuilder {
    self.waves.push(BuilderWave {
      velocity: self.velocity,
      buckets: vec![],
      duration: self.duration,
    });
    self.velocity += self.velocity_increase;
    self
  }
  ///Add a new wave. Duration in seconds.
  pub fn wave_with_duration(mut self, duration: f64) -> WavesBuilder {
    self.waves.push(BuilderWave {
      velocity: self.velocity,
      buckets: vec![],
      duration,
    });
    self.velocity += self.velocity_increase;
    self
  }

  ///Set the duration in seconds of current and for future waves. Default duration is 10 seconds.
  pub fn set_duration(mut self, duration: f64) -> WavesBuilder {
    if let Some(wave) = self.waves.last_mut() {
      wave.duration = duration;
    }
    self.duration = duration;
    self
  }

  ///Set the velocity of current and future waves. This is the base velocity which is increased by `velocity_increase` for each wave added.
  /// `velocity_increase` can be set with `WavesBuilder.set_velocity()`.
  pub fn set_vel(mut self, velocity: f32) -> WavesBuilder {
    if let Some(wave) = self.waves.last_mut() {
      wave.velocity = velocity;
    }
    self.velocity = velocity;
    self
  }

  ///Set the amount the velocity will increase per added wave.
  pub fn set_vel_increase(mut self, velocity_increase: f32) -> WavesBuilder {
    self.velocity_increase = velocity_increase;
    self
  }

  pub fn build(self) -> Vec<Wave> {
    let mut rng = rand::thread_rng();
    let mut spawn_start_offset: f64 = 0.0;
    self
      .waves
      .into_iter()
      .filter(|bw| bw.buckets.len() > 0)
      .enumerate()
      .map(|(wave_index, bw)| {
        //The length of time within which a bucket can be spawned.
        let bucket_duration = bw.duration / bw.buckets.length() as f64;
        let wave = Wave {
          index: wave_index,
          start_time: None,
          duration: bw.duration,
          buckets: bw
            .buckets
            .into_iter()
            .enumerate()
            .map(|(i, (kind, count))| {
              let offset_start = spawn_start_offset + bucket_duration * i as f64;
              let offset_end = spawn_start_offset + bucket_duration * (i + 1) as f64;
              let spawn_offset = rng.gen_range(offset_start..offset_end);
              WaveBucket {
                kind,
                count,
                spawn_offset,
              }
            })
            .rev()
            .collect(),
          velocity: bw.velocity,
        };
        spawn_start_offset += bw.duration;
        return wave;
      })
      .collect()
  }
}

pub fn create_waves() -> Vec<Wave> {
  WavesBuilder::new(40.0)
    .set_vel_increase(2.0)
    .set_duration(6.0)
    .wave()
    .add(TargetKind::Regular, 1)
    .wave_with_duration(2.0)
    .add(TargetKind::Regular, 2)
    .wave_with_duration(10.0)
    .add(TargetKind::Regular, 4)
    .add(TargetKind::Regular, 2)
    .add(TargetKind::Regular, 1)
    .set_vel_increase(8.0)
    .wave()
    .add(TargetKind::Regular, 4)
    .wave()
    .add(TargetKind::Regular, 4)
    .wave()
    .add(TargetKind::Regular, 4)
    .wave()
    .set_vel(1000.0)
    .add(TargetKind::Regular, 100)
    .build()
}
