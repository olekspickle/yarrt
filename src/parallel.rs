use crate::{scene::Scene, Surface};
use lazy_static::lazy_static;
use rand::{rngs::ThreadRng, thread_rng};
use rayon::prelude::*;
use std::cell::Cell;

pub const DIM: f32 = 200.0;

pub const WORLD: Cell<Vec<Box<dyn Surface>>> = Cell::new(vec![]);
pub const RNG: Cell<ThreadRng> = Cell::new(thread_rng());
pub const HEIGHT: f32 = DIM;
pub const WIDTH: f32 = DIM * 2.0;
