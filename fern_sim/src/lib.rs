pub struct Fern {
  pub size: f64,
  pub growth_rate: f64
}

// Simulate a fern growing for one day.
fn grow_fern(fern: &mut Fern) {
  fern.size *= 1.0 + fern.growth_rate;
}

// Run a fern simulation for some number of days.
pub fn run_simulation(fern: &mut Fern, days: usize) {
  for _ in 0..days {
    grow_fern(fern);
  }
}
