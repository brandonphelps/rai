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
    /// Number of generations to run the simulation for.
    pub generation_count: usize, 
    pub parent_selection_count: usize, 
}

/// This trait provides a global storage mechanism for items you'd like to have
pub trait GAStorage {
    // empty
}

/// @brief container instance of all the data associated to the
/// GA at a given point in time. Holds data that is used for each of the call backs
/// Each callback should use the functions provided to update the instance. 
/// one should provide the individual container item. 
/// individuals are lifetimed to the GAState and should provide a Default func, 
// todo: use the where stuff to constrain the individualT as appropriate. 
// todo; can IndividualT be removed and replaced with IndividualTrait?
pub struct GAFunctors<IndividualT> where IndividualT : IndividualTrait {
    // fitness_func: Box<dyn Fn(&IndividualT) -> f32>,
    // on_start: fn(&mut Self, &mut Vec<IndividualT>) -> f32,
    // on_fitness: fn(&mut Self, &IndividualT) -> f32,
    on_start: fn(&mut Vec<IndividualT>) -> f32,
    on_fitness: fn(&IndividualT) -> f32,
    on_parents: fn(&mut dyn GAStorage) -> f32,
}

fn empty_start<T>( indi: &mut Vec<T> ) -> f32 where T: IndividualTrait {
    0.0
}

fn empty_fitness<T>( ind: &T) -> f32 where T: IndividualTrait {
    0.0
}

fn empty_parents(storage: &mut dyn GAStorage) -> f32 {
    0.0
}


impl<IndividualT> GAFunctors<IndividualT> where IndividualT: IndividualTrait {
    pub fn new() -> Self {
	GAFunctors {
	    on_start: empty_start, 
	    on_fitness: empty_fitness,
	    on_parents: empty_parents,
	}
    }
}

// todo: need some generic global storage for some EA systems. 
pub fn run_ea<T, Storage: GAStorage>(ga_params: &GAParams, ga_storage: &mut Storage,
			  ga_functors: &mut GAFunctors::<T>) -> ()
where T: IndividualTrait 
{
    let mut individuals = Vec::<T>::new();

    // startup & initialization
    (ga_functors.on_start)(&mut individuals);
    // produce generation (each individual should provide #[default]
    // calculate fitness

    for _current_gen in 0..ga_params.generation_count { 
	let mut fitness_vec = Vec::<f32>::new();
	for individual in individuals.iter() {
	    let tmp = (ga_functors.on_fitness)(&individual);
	    fitness_vec.push(tmp);
	}

	// parent selection algorithm.

	let mut parents = Vec::<&T>::new();
	
	// todo:replace with actual parent selection algorithm.
	// allow for parent selection process to be selected from list type. 
	for individual in 0..10 {
	    parents.push(&individuals[individual]);
	}

	(ga_functors.on_parents)(ga_storage);
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

	x1: f32,
	x2: f32,
	x3: f32,
	x4: f32,
	x5: f32,
	x6: f32,
    }
    
    impl IndividualTrait for test_individual {
	fn default() -> Self {
	    println!("Generating default individual");
	    test_individual { w1: 0.0, w2: 0.0, w3 : 0.0, w4: 0.0, w5: 0.0, w6: 0.0,
			      x1: 0.0, x2: 0.0, x3 : 0.0, x4: 0.0, x5: 0.0, x6: 0.0 }
	}

	fn fitness(&self) -> f32 {
	    self.w1 * self.x1 + self.w2 * self.x2 + self.w3 * self.x3 + self.w4 * self.x4 + self.w5 * self.x5 + self.w6 * self.x6
	}
    }

    fn test_on_start<T>(pop: &mut Vec<T>) -> f32 where T: IndividualTrait {
	for i in 0..100 {
	    pop.push(T::default());
	}
	return 0.0;
    }

    fn test_on_fitness<T>(individ: &T) -> f32 where T: IndividualTrait {
	return individ.fitness();
    }


    struct CustomStorage {
	pub gen_count: u32,
    }

    impl GAStorage for CustomStorage {

    }

    #[test]
    fn test_playground() {
	let mut ga_functors = GAFunctors::<test_individual>::new();
	let ga_params = GAParams { pop_size: 10,
				   generation_count: 10,
				   parent_selection_count: 2 };
	let mut storage = CustomStorage { gen_count: 0 };
	ga_functors.on_start = test_on_start;
	run_ea::<test_individual, CustomStorage>(&ga_params, &mut storage,
						 &mut ga_functors);
	assert!(false);
    }
}
