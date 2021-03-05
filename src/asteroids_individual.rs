#![allow(dead_code)]

use crate::nn::Network;
use crate::neat::InnovationHistory;
use crate::distro::asteroids_fitness;

// dono bout this. 
// trait FitnessFunctor {
//     fn name(&self) -> String,
//     fn functor(&self) -> impl Fn(),
// }

#[derive(Clone)]
pub struct AsteroidsPlayer {

    // thing of interest.
    brain: Network,
    // whats the diff between dyn Fn(blah blah) and
    // fn9blah blah 
    fitness_func: fn(&Network) -> f64,
    fitness_func_name: String,
}

impl AsteroidsPlayer {
    pub fn new() -> Self {
	// note 8, 3 (input, output) must align with innovation history below. 
	Self { brain: Network::new(8, 3, true),
	       fitness_func: asteroids_fitness,
	       fitness_func_name: String::from("rasteroids"),
	} 
    }

    pub fn fitness(&mut self) -> f64 {
	asteroids_fitness(&self.brain)
    }

    pub fn mutate(&self, _inno: &mut InnovationHistory) -> Self {
	Self::new()
    }
}


pub struct AsteroidsStorage  {
    inno_history: InnovationHistory,
}


impl AsteroidsStorage {
    pub fn new() -> Self {
	Self { inno_history :
	       InnovationHistory::new(8, 3) }
    }
}

