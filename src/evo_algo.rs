#![allow(clippy::unused_unit)]
#![allow(dead_code)]

use std::fmt::Debug;
use std::cmp::Reverse;
use std::collections::HashMap;

// can we get rid of star here?
use rand::prelude::*;
use rand::seq::SliceRandom;

use crate::scheduler::{LocalScheduler, Scheduler};
use crate::distro::{EaFuncMap};

// use crate::asteroids_individual;

// use rand::prelude::*;
// use rand::seq::SliceRandom;

// pub trait Individual {
//     // can this return just a numeric traited instance?
//     // post calculated fitness.
//     fn fitness(&self) -> f64;
//     fn update_fitness(&mut self) -> ();
//     fn print(&self) -> ();
//     fn mutate(&mut self) -> ();
//     // fn crossover(&self, other: Box<dyn Individual>) -> Box<dyn Individual>;
// }


pub trait FitnessFunc {
    type Input;
    
    fn fitness_func(&self, indi: &Self::Input) -> f64;
    fn fitness_name(&self) -> String;
}


/// @brief container class for the various parameters.
pub struct GAParams {
    // total population per generation.
    pub pop_size: usize,

    // how many offspring a population should generate.
    pub offspring_count: usize,
    
    /// Number of generations to run the simulation for.
    pub generation_count: usize,
    pub parent_selection_count: usize,
}

// /// This trait provides a global storage mechanism for items you'd like to have
// pub trait GAStorage {
//     // empty
// }

// /// @brief container instance of all the data associated to the
// /// GA at a given point in time. Holds data that is used for each of the call backs
// /// Each callback should use the functions provided to update the instance.
// /// one should provide the individual container item.
// /// individuals are lifetimed to the GAState and should provide a Default func,
// // todo: use the where stuff to constrain the individualT as appropriate.
// // todo; can IndividualT be removed and replaced with IndividualTrait?
// pub struct GAFunctors<IndividualT, Storage>
// {
//     // on_start: fn(&mut Self, &mut Vec<IndividualT>) -> f32,
//     // on_fitness: fn(&mut Self, &IndividualT) -> f32,
//     on_start: fn(params: &GAParams, &mut Vec<IndividualT>),

//     // do we even need this at all? if IndividualT is met or constrained to contain
//     //  a fitness func then that is there. 
//     // do we need GAParams here?
//     on_fitness: fn(params: &GAParams, &IndividualT) -> f32,
//     on_parents: fn(&mut Storage, parents: &Vec<&IndividualT>) -> f32,
//     on_crossover: fn(&mut Storage, parents: &Vec<&IndividualT>) -> Vec<IndividualT>,
//     on_mutation: fn(&mut Storage, newly_created_individuals: &Vec<&IndividualT>) -> f32,
//     // on_mutation: fn(&mut Storage, newly_created_individuals, offspring_mutation_size);
//     // on_generation: fn(&mut Storage);

//     // on_stop(last_pop_fitness);
// }

// fn empty_start<T>(params: &GAParams, indi: &mut Vec<T>)
// where
//     T: IndividualTrait,
// {
//     for _i in 0..params.pop_size {
// 	indi.push(T::default());
//     }

//     assert_eq!(indi.len(), params.pop_size);
// }


// fn empty_fitness<T>(params: &GAParams, individual: &T) -> f32
// where
//     T: IndividualTrait,
// {
//     individual.fitness()
// }

// fn empty_parents<T, S>(_storage: &mut S, _parents: &Vec<&T>) -> f32 {
//     0.0
// }

// fn empty_crossover<T, S>(_storage: &mut S, _parents: &Vec<&T>) -> Vec<T>
// where
//     T: IndividualTraitExt
// {
//     let mut new_generation = Vec::<T>::new();

//     let mut rng = rand::thread_rng();

//     if rng.gen::<f64>() < 0.25 {
// 	let p = *_parents.choose(&mut rng).unwrap();
// 	new_generation.push(p.clone());
//     }

//     let p_one = _parents.choose(&mut rng).unwrap();
//     let p_two = _parents.choose(&mut rng).unwrap();

//     new_generation.push(p_one.crossover(&p_two));

//     return new_generation;
// }

// fn empty_mutation<T, S>(_storage: &mut S, _new_individuals: &Vec<&T>) -> f32 {
//     0.0
// }

// impl<IndividualT, Storage> GAFunctors<IndividualT, Storage>
// where
//     IndividualT: IndividualTrait,
// {
//     pub fn new() -> Self {
//         GAFunctors {
//             on_start: empty_start,
//             on_fitness: empty_fitness,
//             on_parents: empty_parents,
// 	    on_crossover: empty_crossover,
// 	    on_mutation: empty_mutation,
//         }
//     }
// }

// pub fn run_ea_simple<T>(ga_params: &GAParams,
// 			on_start: impl Fn()) -> ()
// where
//     T: IndividualTrait
// {
    
// }

// // todo: need some generic global storage for some EA systems.
// pub fn run_ea<T, GAStorage>(
//     ga_params: &GAParams,
//     ga_storage: &mut GAStorage,
//     ga_functors: &mut GAFunctors<T, GAStorage>,
// ) -> ()
// where
//     T: IndividualTrait,
// {
//     let mut individuals = Vec::<T>::new();

//     // startup & initialization
//     (ga_functors.on_start)(&ga_params, &mut individuals);
//     // produce generation (each individual should provide #[default]
//     // calculate fitness

//     for _current_gen in 0..ga_params.generation_count {
//         let mut fitness_vec = Vec::<f32>::new();
//         for individual in individuals.iter() {
//             let tmp = (ga_functors.on_fitness)(&ga_params, &individual);
//             fitness_vec.push(tmp);
//         }

//         // parent selection algorithm.

//         let mut parents = Vec::<&T>::new();

//         // todo:replace with actual parent selection algorithm.
//         // allow for parent selection process to be selected from list type.
//         for individual in 0..10 {
//             parents.push(&individuals[individual]);
//         }

//         (ga_functors.on_parents)(ga_storage, &parents);

// 	(ga_functors.on_crossover)(ga_storage, &parents);
//     }
//     // }
//     // do cross over
//     // on_crossover
//     // do mutation
//     // on_mutation()
//     // on_generation
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     impl IndividualTrait for test_individual {
//         fn default() -> Self {
//             println!("Generating default individual");
//             test_individual {
//                 w1: 3.0,
//                 w2: 1.0,
//                 w3: 5.0,
//                 w4: 5.0,
//                 w5: 1.0,
//                 w6: 3.0,

// 		// todo: make these random.
//                 x1: 0.0,
//                 x2: 0.0,
//                 x3: 0.0,
//                 x4: 0.0,
//                 x5: 0.0,
//                 x6: 0.0,
//             }
//         }

//         fn fitness(&self) -> f32 {
// 	}
//     }

//     impl IndividualTraitExt for test_individual {

// 	fn mutate(&self) -> Self {
// 	    let mut f = self.clone();
// 	    f.x1 += 0.02;
// 	    f.x2 -= 0.1;
// 	    f.x3 += 0.20;
// 	    f.x4 += 0.21;
// 	    f.x5 += 0.30;
// 	    f.x6 += 0.32;
// 	    return f;
// 	}

// 	fn crossover(&self, other: &Self) -> Self {
// 	    let mut f = self.clone();
// 	    f.x1 = (other.x1 + self.x1 ) / 2.0;
// 	    f.x2 = (other.x2 + self.x2 ) / 2.0;
// 	    f.x3 = (other.x3 + self.x3 ) / 2.0;
// 	    f.x3 = (other.x3 + self.x3 ) / 2.0;
// 	    f.x5 = (other.x5 + self.x5 ) / 2.0;
// 	    f.x6 = (other.x6 + self.x6 ) / 2.0;
// 	    return f;
// 	}
//     }

//     struct CustomStorage {
//         pub gen_count: u32,

// 	// initial value of weight two. 
// 	pub init_w2: f32,
//     }

//     impl GAStorage for CustomStorage {}

//     // this seems kinda funky, but note that on_parents is of <T, S> but since we
//     // specify the S here as CustomStorage Concrete class we all g. 
//     fn test_on_parents<T>(storage: &mut CustomStorage, parents: &Vec<&T>) -> f32
//     where
//         T: IndividualTrait,
//     {
//         storage.gen_count += 1;
//         return 0.0;
//     }

//     #[test]
//     fn test_playground() {
//         let mut ga_functors = GAFunctors::<test_individual, CustomStorage>::new();
//         let mut storage = CustomStorage { gen_count: 0, init_w2: 3.0 };
//         ga_functors.on_parents = test_on_parents;
//         run_ea::<test_individual, CustomStorage>(&ga_params,
// 						 &mut storage,
// 						 &mut ga_functors);

// 	println!("Gen Count: {}", storage.gen_count);

//         assert!(false);
//     }




//     #[test]
//     fn test_asteroids_playground() {

//     }

// }


// second shot



// RANDOM parent selction
fn select_parents<'a, Individual>(params: &GAParams,
			      fitness: &Vec<f64>,
			      population: &'a Vec<&Individual>) -> Vec<&'a Individual> {
    let mut parents = Vec::<&'a Individual>::new();

    let mut rng = rand::thread_rng();

    for _ in 0..params.parent_selection_count {
	let p = population.choose(&mut rng).unwrap();
	parents.push(p);
    }

    return parents;
}

struct IndiFit<Individual> where Individual: Debug {
    sol: Individual,
    fitness: f64,
}

impl<Individual> IndiFit<Individual> where Individual: Debug{
    fn new(sol: Individual) -> Self {
	Self { sol: sol, fitness: 0.0 }
    }
}

fn run_ea<Individual, Storage>(params: &GAParams,
			       fitness_name: String,
			       on_fitness: fn(&Individual) -> f64,
			       on_crossover: fn(&GAParams, &Vec<&Individual>) -> Vec<Individual>,
			       on_mutate: fn(&GAParams, &mut Storage, &Individual) -> Individual,
			       scheduler: &mut LocalScheduler) -> ()
where
    Individual: Default + Debug + FitnessFunc,
    Storage: Default
{

    // key is the individual, the value is the fitness of said individual
    let mut individuals = Vec::<IndiFit<Individual>>::new();
    let mut storage = Storage::default();

    for _ in 0..params.pop_size { 
	individuals.push(IndiFit::new(Individual::default()));
    }

    // do fitness calculation. 
    for indivi in individuals.iter_mut() {
	indivi.fitness = on_fitness(&indivi.sol);
    }

    for _current_generation in 0..params.generation_count { 
	let mut individuals_fitness = Vec::<f64>::new();
	let mut indivds = Vec::<&Individual>::new();
	
	for indiv in individuals.iter() {
	    individuals_fitness.push(indiv.fitness);
	    indivds.push(&indiv.sol);
	}

	let parents = select_parents(&params, &individuals_fitness,
				     &indivds);

	let offspring = on_crossover(&params, &parents);
	for child in offspring.iter() {
	    let tmp_p = on_mutate(&params, &mut storage, child);
	    individuals.push(IndiFit::new(tmp_p));
	}

	// do fitness calculation. 
	for indivi in individuals.iter_mut() {
	    indivi.fitness = on_fitness(&indivi.sol);
	}

	let mut total_fitness = 0.0;
	for i in individuals.iter() {
	    total_fitness += i.fitness;
	}

	let average_fitness = total_fitness / params.pop_size as f64;
	// cull population
	individuals.sort_by_key(|indivi| Reverse((indivi.fitness * 1000.0) as i128));
	individuals.truncate(params.pop_size as usize);
    }
    individuals.sort_by_key(|indivi| Reverse((indivi.fitness * 1000.0) as i128));
    println!("Top indivi");
    let mut j = 0;
    for i in individuals.iter() {
	println!("{:#?} -> {}", i.sol, i.fitness);
	j += 1;
	if j > 10 {
	    break;
	}
    }
}
    


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct TestIndividual {
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

    impl Default for TestIndividual {
	fn default() -> Self {
	    let mut rng = rand::thread_rng();
	    
	    Self {
		w1: 4.0,
		w2: -2.0,
		w3: 3.5,
		w4: 5.0,
		w5: -11.0,
		w6: -4.7,
		x1: rng.gen::<f32>(),
		x2: rng.gen::<f32>(),
		x3: rng.gen::<f32>(),
		x4: rng.gen::<f32>(),
		x5: rng.gen::<f32>(),
		x6: rng.gen::<f32>()
	    }
	}
    }

    

    #[derive(Default)]
    struct GStorage { }
    
    fn ind_fitness(individual: &TestIndividual) -> f64 {
        let p = (individual.w1 * individual.x1
		 + individual.w2 * individual.x2
		 + individual.w3 * individual.x3
		 + individual.w4 * individual.x4
		 + individual.w5 * individual.x5
		 + individual.w6 * individual.x6) as f64;
	let fitness = (44.0 - p).abs();
	if fitness == 0.0 {
	    return 100000000000000.0;
	}
	else{
	    return 1.0 / fitness;
	}
    }

    fn ind_mutate(params: &GAParams, storage: &mut GStorage, indivi: &TestIndividual) -> TestIndividual {

	let mut params = Vec::<f64>::new();

	for i in 0..6 {
	    let new_x = 0.0;

	}


	TestIndividual {
	    w1: indivi.w1,
	    w2: indivi.w2,
	    w3: indivi.w3,
	    w4: indivi.w4,
	    w5: indivi.w5,
	    w6: indivi.w6,
	    x1: indivi.x1 + 0.2,
	    x2: indivi.x2 + 0.3,
	    x3: indivi.x3 + 0.2,
	    x4: indivi.x4 + 0.1,
	    x5: indivi.x5 + 0.1,
	    x6: indivi.x6 + 0.2,
	}
    }

    fn ind_crossover(params: &GAParams, parents: &Vec<&TestIndividual>) -> Vec<TestIndividual> {
	let mut new_offspring = Vec::<TestIndividual>::new();
	let mut rng = rand::thread_rng();

	while new_offspring.len() < params.offspring_count { 
	    // single point crossover
	    if rng.gen::<f64>() < 0.25 {
		let mut params_x: [f32; 6] = [0.0; 6];
		let indivi_one = *parents.choose(&mut rng).unwrap();
		let indivi_two = *parents.choose(&mut rng).unwrap();

		let point_p: u32 = rng.gen_range(0, 7);
		// parent 1.
		if point_p > 0 {
		    params_x[0] = indivi_one.x1;
		} else {
		    params_x[0] = indivi_two.x1;
		}
		if point_p > 1 {
		    params_x[1] = indivi_one.x2;
		} else {
		    params_x[1] = indivi_two.x2;
		}
		if point_p > 2 {
		    params_x[2] = indivi_one.x3;
		} else {
		    params_x[2] = indivi_two.x3;
		}
		if point_p > 3 {
		    params_x[3] = indivi_one.x4;
		} else {
		    params_x[3] = indivi_two.x4;
		}
		if point_p > 4 {
		    params_x[4] = indivi_one.x5;
		} else {
		    params_x[4] = indivi_two.x5;
		}
		if point_p > 5 {
		    params_x[5] = indivi_one.x6;
		} else {
		    params_x[5] = indivi_two.x6;
		}

		new_offspring.push( TestIndividual {
		    w1: indivi_one.w1,
		    w2: indivi_one.w2,
		    w3: indivi_one.w3,
		    w4: indivi_one.w4,
		    w5: indivi_one.w5,
		    w6: indivi_one.w6,
		    x1: params_x[0],
		    x2: params_x[1],
		    x3: params_x[2],
		    x4: params_x[3],
		    x5: params_x[4],
		    x6: params_x[5],
		});

	    } else {
		
		if rng.gen::<f64>() < 0.5 {
		    let p = *parents.choose(&mut rng).unwrap();
		    new_offspring.push(p.clone());
		} else {
		    let indivi_one = *parents.choose(&mut rng).unwrap();
		    let indivi_two = *parents.choose(&mut rng).unwrap();

		    let p = TestIndividual {
			w1: indivi_one.w1,
			w2: indivi_one.w2,
			w3: indivi_one.w3,
			w4: indivi_one.w4,
			w5: indivi_one.w5,
			w6: indivi_one.w6,
			x1: (indivi_one.x1 +  indivi_two.x1) / 2.0,
			x2: (indivi_one.x2 +  indivi_two.x2) / 2.0,
			x3: (indivi_one.x3 +  indivi_two.x3) / 2.0,
			x4: (indivi_one.x4 +  indivi_two.x4) / 2.0,
			x5: (indivi_one.x5 +  indivi_two.x5) / 2.0,
			x6: (indivi_one.x6 +  indivi_two.x6) / 2.0,
		    };
		    new_offspring.push(p);
		}
	    }
	}
	return new_offspring;
    }


    #[test]
    fn test_playground() {
        let ga_params = GAParams {
            pop_size: 10,
	    offspring_count: 200,
            generation_count: 10000,
            parent_selection_count: 10,
        };
	

	let mut scheduler = LocalScheduler::new();

	struct P<F> where F: FitnessFunc {
	    m: HashMap::<String, F>,
	}


	struct PizzaIndividual {
	    pizza_count: u8,
	}

	struct PizzaFitness;

	impl FitnessFunc for PizzaFitness {
	    type Input = PizzaIndividual;

	    fn fitness_func(&self, pizza: &PizzaIndividual) -> f64 {
		pizza.pizza_count as f64
	    }

	    fn fitness_name(&self) -> String {
		String::from("pizza")
	    }
	}

	struct TestFitness;

	impl FitnessFunc for TestFitness {
	    type Input = TestIndividual;
	    fn fitness_func(&self, ind: &TestIndividual) -> f64 {
		ind_fitness(&ind)
	    }

	    fn fitness_name(&self) -> String {
		String::from("Math")
	    }
	}

	impl FitnessFunc for PizzaIndividual {
	    fn fitness_func(&self) -> f64 {
		// more pizza is better!
		self.pizza_count as f64
	    }

	    fn fitness_name(&self) -> String {
		String::from("Pizza")
	    }
	}

	struct FuncMapper {
	    funcs: HashMap::<String, Box<dyn FitnessFunc>>,
	}

	impl FuncMapper {
	    pub fn new() -> Self {
		FuncMapper { funcs: HashMap::new() } 
	    }

	    pub fn add_func(&mut self, name: String, f: Box<dyn FitnessFunc>) -> () {
		self.funcs.insert(name, f);
	    }

	    pub fn call_func(&self, name: String) -> f64 {
		self.funcs.get(&name).unwrap().fitness_func()
	    }

	    pub fn get_func(&self, name: String) -> &Box<dyn FitnessFunc> {
		self.funcs.get(&name).unwrap()
	    }
	}

	pub trait Draw {
	    fn draw(&self);
	}

	pub struct Screen {
	    pub components: HashMap<String, Box<dyn FitnessFunc>>,
	}

	

	let mut m = FuncMapper::new();
	m.add_func(String::from("mah_fit"),
		   Box::<TestFitness>::new(TestFitness { }));
	m.add_func(String::from("pizza_fit"), Box::<PizzaFitness>::new(PizzaFitness { }));
	m.call_func(String::from("mah_fit"));
	m.call_func(String::from("pizza_fit"));

	let p = m.get_func(String::from("mah_fit"));

	println!("Calling: {}, {}", p.fitness_name(), p.fitness_func());

	impl FitnessFunc for TestIndividual {
	    fn fitness_func(&self) -> f64 {
		ind_fitness(&self)
	    }

	    fn fitness_name(&self) -> String {
		String::from("Test")
	    }
	}

	// m.add_func(String::from("TestInd"),
	// 	   Box::<TestIndividual>::new(TestIndividual { }));



	assert!(false);

	run_ea::<TestIndividual, GStorage>(&ga_params,
					   String::from("math_fit"),
					   ind_fitness,
					   ind_crossover,
					   ind_mutate,
					   &mut scheduler);
	assert!(false);
    }
}




