#![allow(dead_code)]

use std::fmt::Debug;
use std::cmp::Reverse;


// can we get rid of star here?
use rand::prelude::*;
use rand::seq::SliceRandom;

use crate::promise::{LocalScheduler, Scheduler};

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
    Individual: Default + Debug,
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

	// todo: build some sort of results type such that
	// we don't print here. 
	let average_fitness = total_fitness / params.pop_size as f64;
	println!("Average fitness: {}", average_fitness);
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
	

	let mut scheduler = LocalScheduler { };

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

