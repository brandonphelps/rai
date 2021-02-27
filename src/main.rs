#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]
use prgrs::{Length, Prgrs};
use rand::distributions::{Distribution, Normal};
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::cmp::Reverse;
use std::env;
use std::fs;
use std::{thread, time};


use serde::{Deserialize, Serialize};
use serde_json::Result;

mod scheduler;
mod distro;
mod evo_algo;
mod hrm;
mod neat;
mod nn;

use rasteroids::asteroids;
use rasteroids::collision;

use crate::scheduler::{Scheduler, LocalScheduler};

use std::collections::HashMap;
use std::time::Instant;

use std::sync::atomic::{AtomicUsize, Ordering};




/// Given the total fitness, species' fitness, and total pop, generate a total number of
/// items
// todo: move this. 
fn num_child_to_make(total_fitness: f64, species_fitness: f64, total_population: u64) -> u64 {
    println!(
        "Total: ({}) Spec: ({}) Pop: ({})",
        total_fitness, species_fitness, total_population
    );
    assert!(total_fitness >= species_fitness);
    ((species_fitness / total_fitness) * total_population as f64) as u64
}

fn run_ea(
    input_count: u32,
    output_count: u32,
    pop_count: u64,
    iter_count: u64,
    results_folder: String,
    fitness_func: &impl Fn(&nn::Network) -> f64,
) -> () {
    let mut average_history_per_iter: Vec<f64> = Vec::new();

    // initializeation.
    // population holder.
    let mut specific_pop: Vec<nn::Network> = Vec::new();
    let mut individual = nn::Network::new(input_count, output_count, true);
    // fitness evaluation
    let mut innovation_history = neat::InnovationHistory {
        global_inno_id: (input_count * output_count) as usize,
        conn_history: vec![],
    };

    // run the first one locally.
    let fitness_res = fitness_func(&individual);
    individual.fitness = fitness_res;

    for _ in 0..pop_count + 1 {
        specific_pop.push(individual.clone());
    }

    for generation in 0..iter_count {
        let gen_folder = format!("{}/{}", results_folder, generation);
        fs::create_dir_all(gen_folder.clone()).expect("Failed to create results folder");

        println!("Generation: {}", generation);

        // move to speciate function
        // specization. divide the population into different species.
        // why can't this forloop be outside this forloop? something
        // about the specific_pop updating is mutable borrow after an immutable barrow on something?

        let mut species = neat::speciate(&specific_pop);
        let species_count = species.len();
        println!("Species count: {}", species_count);

        let mut offspring = Vec::new();

        // there is prob some vector function for this or something with a closure?
        let mut total_fitness = 0.0;
        for ind in specific_pop.iter() {
            total_fitness += ind.fitness();
        }
        let average_fit = total_fitness / (specific_pop.len() as f64);

        println!("Fitness ({}), ({})", total_fitness, average_fit);

        // generate offsprint from each of the species.
        // the number of offspring depends on the average fitness of the species.
        for spec in species.iter() {
            // add in the champ of the species in.
            offspring.push(spec.champion.unwrap().clone());
            let spec_fitness = spec.total_fitness();
            let num_children = num_child_to_make(total_fitness, spec_fitness, pop_count);

            for _child_num in 0..num_children {
                let mut new_child = spec.generate_offspring(&innovation_history).clone();
                new_child.mutate(&mut innovation_history);
                // assert_eq!(node_per_layer(new_child.
                offspring.push(new_child);
            }
        }

        let start = Instant::now();
        {
            //let mut schedu = Scheduler::new("192.168.1.77", 11300);
	    let mut schedu = LocalScheduler::new();
            for off_p in offspring.iter_mut() {
                // fitness_func(&mut new_child);
                // evaluate_individual(&mut new_child, fitness_func);
		println!("Scheduling job");
                schedu.schedule_job(off_p, &"rasteroids".to_string());
            }

            schedu.wait();
        }
        let duration = start.elapsed();
        if duration.as_secs() != 0 {
            println!(
                "Fitness per second: {}",
                offspring.len() as f64 / duration.as_secs() as f64
            );
        }

        species.clear();

        for (index, ind) in specific_pop.iter().enumerate() {
            let j = serde_json::to_string(&ind).unwrap();
            std::fs::write(format!("{}/{}", gen_folder, index), j).expect("Unable to write");
        }

        specific_pop.append(&mut offspring);

        // // cull population
        specific_pop.sort_by_key(|indiv| Reverse((indiv.fitness() * 1000.0) as i128));
        specific_pop.truncate(pop_count as usize);

        assert!(specific_pop.len() == pop_count as usize);
        println!(
            "Species({}) average fitness {} number of innovations: {}",
            species_count,
            average_fit,
            innovation_history.conn_history.len()
        );
        average_history_per_iter.push(average_fit / (specific_pop.len() as f64));
    }

    specific_pop.sort_by_key(|indiv| Reverse((indiv.fitness() * 1000.0) as i128));
    let _top = &mut specific_pop[0];
}

#[cfg(not(feature="gui"))]
fn main() -> std::result::Result<(), String> {
    let population_count = 400;
    let max_iter_count = 10000;
    let input_node_count = 8;
    let output_node_count = 3;

    let _args: Vec<_> = env::args().collect();

    use chrono::{Datelike, Local, Timelike, Utc};

    let now = Utc::now();
    let (is_pm, mut now_hour) = now.hour12();
    if is_pm {
        now_hour += 12;
    }
    let folder_time = format!("{}{}{}", now_hour, now.minute(), now.second());

    use std::process::Command;

    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("Failed to get git hash");

    let runner_version = match std::str::from_utf8(&output.stdout[0..6]) {
        Ok(v) => v,
        Err(_e) => panic!("Failed to get runner version"),
    };

    let results_folder = format!("results/asteroids/{}_{}", folder_time, runner_version);
    println!("Storing results in {}", results_folder);
    match fs::create_dir_all(results_folder.clone()) {
        Err(e) => println!("Failed to create folder: {}", e),
        _ => (),
    }

    run_ea(
        input_node_count,
        output_node_count,
        population_count,
        max_iter_count,
        results_folder,
        &distro::asteroids_fitness,
    );

    Ok(())
}


// todo look at this bench amrk thing https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust

#[cfg(test)]
mod tests {
    use super::*;

    /// test if the innovation history with edge and node additions works.
    #[test]
    fn test_innovation_history() {
        let mut innovation_history = neat::InnovationHistory {
            global_inno_id: (3 * 4),
            conn_history: vec![],
        };

        let mut network = nn::Network::new(2, 4, true);

        let node2 = network.add_node(0, 0.4, 0.5, Some(&mut innovation_history));
        // adding nodes adds two edges, therefor there are two mutations and thus we expect 2 items.
        assert_eq!(innovation_history.conn_history.len(), 2);
        let _node_ref = &network.nodes[node2 as usize];

        let _edge1 = network.add_connection(1, node2 as usize, 1.0, Some(&mut innovation_history));
        println!("{:#?}", network);
        println!("{:#?}", innovation_history);
    }

    #[test]
    fn test_species() {
        let num_inputs = 2;
        let num_outputs = 4;
        let network = nn::Network::new(num_inputs, num_outputs, true);
        let network_two = nn::Network::new(num_inputs, num_outputs, true);

        let _innovation_history = neat::InnovationHistory {
            global_inno_id: ((num_inputs + 1) * num_outputs) as usize,
            conn_history: vec![],
        };

        assert_eq!(
            0,
            neat::Species::get_excess_disjoint(&network.edges, &network_two.edges)
        );

        let network_three = nn::Network::new(num_inputs, num_outputs, false);
        assert_eq!(
            ((num_inputs + 1) * num_outputs) as usize,
            neat::Species::get_excess_disjoint(&network.edges, &network_three.edges)
        );

        assert_eq!(
            0.0,
            neat::Species::get_average_weight_diff(&network.edges, &network_two.edges)
        );

        let mut spec = neat::Species::new(0.5, 0.4, 1.2);

        let test_network = network.clone();
        spec.set_champion(&test_network);
        assert!(spec.same_species(&network.edges));
        assert!(spec.same_species(&network_two.edges));
        assert!(!spec.same_species(&network_three.edges));
    }

    #[test]
    fn test_speciate() {
        let num_inputs = 2;
        let num_outputs = 4;
        let network = nn::Network::new(num_inputs, num_outputs, true);
        let mut network_two = nn::Network::new(num_inputs, num_outputs, true);

        let mut innovation_history = neat::InnovationHistory {
            global_inno_id: ((num_inputs + 1) * num_outputs) as usize,
            conn_history: vec![],
        };

        assert_eq!(
            0,
            neat::Species::get_excess_disjoint(&network.edges, &network_two.edges)
        );
        assert_eq!(
            0.0,
            neat::Species::get_average_weight_diff(&network.edges, &network_two.edges)
        );

        let network_three = nn::Network::new(num_inputs, num_outputs, false);
        assert_eq!(
            ((num_inputs + 1) * num_outputs) as usize,
            neat::Species::get_excess_disjoint(&network.edges, &network_three.edges)
        );

        network_two.add_node(0, 0.2, 0.4, Some(&mut innovation_history));
        assert_eq!(
            2,
            neat::Species::get_excess_disjoint(&network.edges, &network_two.edges)
        );
        network_two.add_node(5, 0.2, 0.4, Some(&mut innovation_history));
        assert_eq!(
            4,
            neat::Species::get_excess_disjoint(&network.edges, &network_two.edges)
        );

        println!("Network one");
        network.pretty_print();
        println!("Network two");
        network_two.pretty_print();

        let mut spec = neat::Species::new(0.5, 0.4, 1.2);

        let tmp_test = network.clone();
        spec.set_champion(&tmp_test);
        assert!(spec.same_species(&network.edges));
        assert!(!spec.same_species(&network_two.edges));
        assert!(!spec.same_species(&network_three.edges));
    }

    #[test]
    fn test_child_num_people() {
        let total_fitness = 100.0;
        let total_pop = 100;

        assert_eq!(num_child_to_make(total_fitness, 100.0, total_pop), 100);

        // if a species has half the total pop then it should contribute to half the population.
        assert_eq!(num_child_to_make(total_fitness, 50.0, total_pop), 50);

        assert_eq!(num_child_to_make(total_fitness, 2.0, total_pop), 2);
        assert_eq!(num_child_to_make(total_fitness, 50.0, 200), 100);
    }
}
