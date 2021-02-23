#![allow(clippy::unused_unit)]

pub trait Individual {
    // can this return just a numeric traited instance?
    // post calculated fitness.
    fn fitness(&self) -> f64;
    fn update_fitness(&mut self) -> ();
    fn print(&self) -> ();
    fn mutate(&mut self) -> ();
    // fn crossover(&self, other: Box<dyn Individual>) -> Box<dyn Individual>;
}

pub trait Crossover<Rhs = Self> {
    type Output;

    fn crossover(&self, rhs: &Rhs) -> Self::Output;
}

pub trait RandomDefault: Default  {
    // empty
}

pub trait Fitness {
    fn fitness(&self) -> f32;
}

pub trait IndividualTrait {
    fn fitness(&self) -> f32;
    fn default() -> Self;
}

/// @brief container class for the various parameters.
pub struct GAParams {
    pub pop_size: usize,

}

/// @brief container instance of all the data associated to the
/// GA at a given point in time. Holds data that is used for each of the call backs
/// Each callback should use the functions provided to update the instance. 
/// one should provide the individual container item. 
/// individuals are lifetimed to the GAState and should provide a Default func, 
// todo: use the where stuff to constrain the individualT as appropriate. 
pub struct GAState<IndividualT> where IndividualT : IndividualTrait {
    params: GAParams,
    // fitness_func: Box<dyn Fn(&IndividualT) -> f32>,
    on_start: fn(&mut Self) -> f32,
    on_fitness: fn(&mut Self, &IndividualT) -> f32,
}

fn empty_start<T>(ga: &mut GAState<T>) -> f32 where T: IndividualTrait {
    0.0
}

fn empty_fitness<T>(ga: &mut GAState<T>, ind: &T) -> f32 where T: IndividualTrait {
    0.0
}


impl<IndividualT> GAState<IndividualT> where IndividualT: IndividualTrait {
    pub fn new(pop_size: usize) -> Self {
	GAState {
	    params: GAParams { pop_size: pop_size },
	    on_start: empty_start, 
	    on_fitness: empty_fitness,
	}
    }

    pub fn get_max_pop_count(&self) -> usize {
	self.params.pop_size
    }
}


fn on_start<T>(ga_state: &mut GAState<T>) where T: IndividualTrait {

}

fn on_fitness<T>(ga_state: &mut GAState<T>, individual: &T) -> f32 where T: IndividualTrait {
    0.0
}


pub fn run_ea<T>(gen_count: u16, pop_size: u16) -> ()
where T: IndividualTrait 
{
    let mut individuals = Vec::<T>::new();
    // startup & initialization
    let mut ga_state = GAState::<T>::new(pop_size as usize);

    (ga_state.on_start)(&mut ga_state);
    // produce generation (each individual should provide #[default]
    // calculate fitness
    
    for individual in individuals.iter() {
	(ga_state.on_fitness)(&mut ga_state, &individual);
    }
    // }
    // selection of parents
    // on_parents ()
    // do cross over
    // on_crossover
    // do mutation
    // on_mutation()
    // on_generation
}


#[cfg(test)]
mod tests {
    use super::*;

    struct test_individual {
	w1: f32,
	w2: f32,
	w3: f32,
	w4: f32,
	w5: f32,
	w6: f32,
    }
    
    impl Default for test_individual {
	fn default() -> Self {
	    println!("Generating default individual");
	    test_individual { w1: 0.0, w2: 0.0, w3 : 0.0, w4: 0.0, w5: 0.0, w6: 0.0 }
	}
    }

    #[test]
    fn test_playground() {
	let t = test_individual::default();
	run_ea::<test_individual>(10, 20);
	assert!(false);
    }
}
