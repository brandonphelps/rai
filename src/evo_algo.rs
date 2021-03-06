#![allow(dead_code)]

use std::cmp::Reverse;
use std::fmt::Debug;

use std::collections::HashMap;

// can we get rid of star here?
use rand::prelude::*;
use rand::seq::SliceRandom;

use crate::individual::Individual;
#[allow(unused_imports)]
use crate::promise::{LocalScheduler, Scheduler};

use crate::asteroids_individual::AsteroidsPlayer;
use crate::neat;
use crate::nn::Network;

use std::task::Poll;

fn num_child_to_make(total_fitness: f64, species_fitness: f64, total_population: u64) -> u64 {
    println!(
        "Total: ({}) Spec: ({}) Pop: ({})",
        total_fitness, species_fitness, total_population
    );
    assert!(total_fitness >= species_fitness);
    ((species_fitness / total_fitness) * total_population as f64) as u64
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

// RANDOM parent selction
fn select_parents<'a, Individual>(
    params: &GAParams,
    _fitness: &Vec<f64>,
    population: &'a Vec<&Individual>,
) -> Vec<&'a Individual> {
    let mut parents = Vec::<&'a Individual>::new();

    let mut rng = rand::thread_rng();

    for _ in 0..params.parent_selection_count {
        let p = population.choose(&mut rng).unwrap();
        parents.push(p);
    }

    return parents;
}

struct IndiFit<Individual>
where
    Individual: Debug,
{
    sol: Individual,
    fitness: f64,
}

impl<Individual> IndiFit<Individual>
where
    Individual: Debug,
{
    fn new(sol: Individual) -> Self {
        Self {
            sol: sol,
            fitness: 0.0,
        }
    }
}

trait ExtractBrain {
    fn get_brain(&self) -> Network;
}

// Storage == neat::InnovationHistory
fn species_crossover<IndividualT>(
    params: &GAParams,
    innovation_history: &mut neat::InnovationHistory,
    current_pop: &Vec<&IndividualT>,
) -> Vec<IndividualT>
where
    IndividualT: Individual + ExtractBrain,
{
    let mut results = Vec::<IndividualT>::new();
    let mut brains = Vec::<Network>::new();

    for i in current_pop.iter() {
        brains.push(i.get_brain());
    }

    let mut species = neat::speciate(&brains);

    let species_count = species.len();
    println!("num species: {}", species_count);

    let mut offspring = Vec::new();

    let mut total_fitness = 0.0;

    println!("Getting fitness of population");
    // todo: don't re calculate fitness.
    for ind in current_pop.iter() {
        total_fitness += ind.fitness();
    }
    println!("Total fitness: {}", total_fitness);

    for spec in species.iter() {
        // add in the champion of the species.
        offspring.push(spec.champion.unwrap().clone());

        let spec_fitness = spec.total_fitness();
        let num_children = num_child_to_make(total_fitness, spec_fitness, params.pop_size as u64);

        for _child_num in 0..num_children {
            let mut new_child = spec.generate_offspring(innovation_history).clone();

            // todo: why does &mut work here?
            new_child.mutate(innovation_history);
            offspring.push(new_child);
        }
    }
    return results;
}

fn generic_offspring_gen<IndividualT>(
    params: &GAParams,
    current_pop: &Vec<IndividualT>,
) -> Vec<IndividualT> {
    let mut results = Vec::<IndividualT>::new();

    while results.len() < params.offspring_count {}

    return results;
}

fn run_ea<IndividualT, Storage, Sched>(
    params: &GAParams,
    storage: &mut Storage,
    on_crossover: fn(&GAParams, &mut Storage, &Vec<&IndividualT>) -> Vec<IndividualT>,
    on_mutate: fn(&GAParams, &mut Storage, &IndividualT) -> IndividualT,
    generate_offspring_func: fn(&GAParms, &mut Storage, &Vec<&IndividualT>) -> Vec<IndividualT>,
    scheduler: &mut Sched,
) -> ()
where
    IndividualT: Default + Debug + Individual,
    Sched: Scheduler<IndividualT>,
{
    // key is the individual, the value is the fitness of said individual
    let mut individuals = Vec::<IndiFit<IndividualT>>::new();
    for _ in 0..params.pop_size {
        individuals.push(IndiFit::new(IndividualT::default()));
    }

    // do fitness calculation.
    for indivi in individuals.iter_mut() {
        indivi.fitness = indivi.sol.fitness();
        println!("Calculating fitness");
    }

    for _current_generation in 0..params.generation_count {
        let mut individuals_fitness = Vec::<f64>::new();
        let mut indivds = Vec::<&IndividualT>::new();

        for indiv in individuals.iter() {
            individuals_fitness.push(indiv.fitness);
            indivds.push(&indiv.sol);
        }

        let offspring = generate_offspring_func(&params, storage, &indivds);

        for child in offspring.iter() {
            let tmp_p = on_mutate(&params, storage, child);
            individuals.push(IndiFit::new(tmp_p));
        }

        {
            let mut results = HashMap::new();
            // do fitness calculation.
            for (index, indivi) in individuals.iter().enumerate() {
                results.insert(index, scheduler.schedule_job(indivi.sol.clone()));
            }

            println!("Waiting for scheduler to finish");
            scheduler.wait();

            for pair in results.iter_mut() {
                match pair.1.poll(scheduler) {
                    Poll::Ready(fitness) => {
                        println!("Getting fitness: {}", fitness);
                        individuals[*pair.0 as usize].fitness = fitness;
                    }
                    Poll::Pending => {
                        panic!("Failed to get fitness")
                    }
                }
            }
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

    impl TestIndividual {
        pub fn crossover(&self, other: &Self) -> Self {
            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() < 0.5 {
                let mut params_x: [f32; 6] = [0.0; 6];
                let point_p: u32 = rng.gen_range(0, 7);
                // parent 1.
                if point_p > 0 {
                    params_x[0] = self.x1;
                } else {
                    params_x[0] = other.x1;
                }
                if point_p > 1 {
                    params_x[1] = self.x2;
                } else {
                    params_x[1] = other.x2;
                }
                if point_p > 2 {
                    params_x[2] = self.x3;
                } else {
                    params_x[2] = other.x3;
                }
                if point_p > 3 {
                    params_x[3] = self.x4;
                } else {
                    params_x[3] = other.x4;
                }
                if point_p > 4 {
                    params_x[4] = self.x5;
                } else {
                    params_x[4] = other.x5;
                }
                if point_p > 5 {
                    params_x[5] = self.x6;
                } else {
                    params_x[5] = other.x6;
                }

                TestIndividual {
                    w1: self.w1,
                    w2: self.w2,
                    w3: self.w3,
                    w4: self.w4,
                    w5: self.w5,
                    w6: self.w6,
                    x1: params_x[0],
                    x2: params_x[1],
                    x3: params_x[2],
                    x4: params_x[3],
                    x5: params_x[4],
                    x6: params_x[5],
                }
            } else {
                TestIndividual {
                    w1: self.w1,
                    w2: self.w2,
                    w3: self.w3,
                    w4: self.w4,
                    w5: self.w5,
                    w6: self.w6,
                    x1: (self.x1 + other.x1) / 2.0,
                    x2: (self.x2 + other.x2) / 2.0,
                    x3: (self.x3 + other.x3) / 2.0,
                    x4: (self.x4 + other.x4) / 2.0,
                    x5: (self.x5 + other.x5) / 2.0,
                    x6: (self.x6 + other.x6) / 2.0,
                }
            }
        }
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
                x6: rng.gen::<f32>(),
            }
        }
    }

    #[derive(Default)]
    struct GStorage {}

    impl Individual for TestIndividual {
        fn fitness(&self) -> f64 {
            let p = (self.w1 * self.x1
                + self.w2 * self.x2
                + self.w3 * self.x3
                + self.w4 * self.x4
                + self.w5 * self.x5
                + self.w6 * self.x6) as f64;
            let fitness = (44.0 - p).abs();
            if fitness == 0.0 {
                return 100000000000000.0;
            } else {
                return 1.0 / fitness;
            }
        }

        fn ea_name(&self) -> String {
            String::from("mathfit")
        }
    }

    fn asteroids_mut(
        _params: &GAParams,
        storage: &mut neat::InnovationHistory,
        indivi: &AsteroidsPlayer,
    ) -> AsteroidsPlayer {
        indivi.clone()
    }

    fn ind_mutate(
        _params: &GAParams,
        storage: &mut GStorage,
        indivi: &TestIndividual,
    ) -> TestIndividual {
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

    // todo: rename
    fn ind_crossover(
        params: &GAParams,
        storage: &mut GStorage,
        individs: &Vec<&TestIndividual>,
    ) -> Vec<TestIndividual> {
        let mut individuals_fitness = Vec::<f64>::new();
        for indiv in individs.iter() {
            individuals_fitness.push(indiv.fitness());
        }

        let parents = select_parents(&params, &individuals_fitness, &individs);

        let mut new_offspring = Vec::<TestIndividual>::new();
        let mut rng = rand::thread_rng();

        while new_offspring.len() < params.offspring_count {
            // single point crossover
            let indivi_one = parents.choose(&mut rng).unwrap();
            let indivi_two = parents.choose(&mut rng).unwrap();
            new_offspring.push(indivi_one.crossover(indivi_two));
        }
        return new_offspring;
    }

    #[test]
    fn test_playground() {
        let ga_params = GAParams {
            pop_size: 10,
            offspring_count: 10,
            generation_count: 10,
            parent_selection_count: 10,
        };

        let mut scheduler = LocalScheduler::<TestIndividual>::new();
        let mut a_scheduler = LocalScheduler::<AsteroidsPlayer>::new();

        let mut innovation_history = neat::InnovationHistory::new(8, 3);

        let mut r_s = GStorage {};

        run_ea::<TestIndividual, GStorage, LocalScheduler<TestIndividual>>(
            &ga_params,
            &mut r_s,
            ind_crossover,
            ind_mutate,
            &mut scheduler,
        );

        impl ExtractBrain for AsteroidsPlayer {
            fn get_brain(&self) -> Network {
                self.brain.clone()
            }
        }

        println!("Asteroids");
        run_ea::<AsteroidsPlayer, neat::InnovationHistory, LocalScheduler<AsteroidsPlayer>>(
            &ga_params,
            &mut innovation_history,
            species_crossover,
            asteroids_mut,
            &mut a_scheduler,
        );

        assert!(false);
    }
}
