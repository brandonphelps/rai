
use crate::nn::Network;
use crate::neat::InnovationHistory;

use crate::distro::asteroids_fitness;

trait FitnessFunctor {
    fn name(&self) -> String,
}

struct AsteroidsPlayer {

    // thing of interest.
    brain: Network,
    fitness_func: dyn FitnessFunctor,
}


#[derive(Clone)]
impl AsteroidsPlayer {
    pub fn new() -> Self {
	// note 8, 3 (input, output) must align with innovation history below. 
	Self { brain: Network::new(8, 3, true) } 
    }

    pub fn fitness(&mut self) -> f32 {
	asteroids_fitness(&mut self.brain);
	return self.brain.fitness;
    }
}


struct AsteroidsStorage  {
    inno_history: InnovationHistory;
}

impl AsteroidsStorage {
    pub fn new() -> Self {
	Self { inno_history :
	       InnovationHistory::new(8, 3) }
	}
    }
}
