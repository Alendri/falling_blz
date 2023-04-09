use rand::{distributions::Uniform, prelude::Distribution};
use std::ops::Range;

pub struct Zone {
  top: isize,
  right: isize,
  bottom: isize,
  left: isize,
  x_between: Uniform<isize>,
  y_between: Uniform<isize>,
}
impl Zone {
  pub fn new(top: isize, right: isize, bottom: isize, left: isize) -> Self {
    let (x_between, y_between) = Zone::create_rng(top..bottom, left..right);
    Zone {
      top,
      right,
      bottom,
      left,
      x_between,
      y_between,
    }
  }
  pub fn update(&mut self, top: isize, right: isize, bottom: isize, left: isize) {
    let (x_between, y_between) = Zone::create_rng(top..bottom, left..right);
    self.top = top;
    self.right = right;
    self.bottom = bottom;
    self.left = left;
    self.x_between = x_between;
    self.y_between = y_between;
  }
}

impl Region for Zone {
  fn get_betweens(&self) -> (&Uniform<isize>, &Uniform<isize>) {
    (&self.x_between, &self.y_between)
  }
}

pub trait Region {
  fn create_rng(x_range: Range<isize>, y_range: Range<isize>) -> (Uniform<isize>, Uniform<isize>) {
    (
      Uniform::try_from(x_range).unwrap(),
      Uniform::try_from(y_range).unwrap(),
    )
  }

  fn get_betweens(&self) -> (&Uniform<isize>, &Uniform<isize>);

  fn get_rand_pt(&self) -> (isize, isize) {
    let (x_between, y_between) = self.get_betweens();
    let mut rng = rand::thread_rng();
    (x_between.sample(&mut rng), y_between.sample(&mut rng))
  }
}
