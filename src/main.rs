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

extern crate beanstalkd;

use beanstalkc::Beanstalkc;

use beanstalkd::Beanstalkd;

use serde::{Deserialize, Serialize};
use serde_json::Result;


use rasteroids::asteroids;
use rasteroids::collision;

mod evo_algo;
mod hrm;
mod neat;
mod nn;
mod distro;

use std::time::{Duration, Instant};
use std::collections::HashMap;

use std::sync::atomic::{AtomicUsize, Ordering};



// fmt cares about build ability or something
struct Scheduler<'a>  {
    current_jobs: Vec<(u128, &'a mut nn::Network)>, 
    job_queue: Beanstalkc,
    next_job_id: u128,
}

impl<'a> Scheduler<'a> {
    // todo: allow for local where beanstalk is not used. 
    pub fn new(host: &str, port: u16) -> Scheduler  {
	let mut p = Beanstalkc::new().host(host).port(port).connect().expect("Connection failed");
	p.watch("results");
	return Scheduler {
	    current_jobs: vec![],
	    job_queue: p,
	    next_job_id: 0,
	};
    }

    /// @param: fitness_func_name name of fitness function to run. 
    pub fn schedule_job(&mut self, individual: &'a mut nn::Network, fitness_func_name: &String) -> () {

	self.job_queue.use_tube(&fitness_func_name);
	let job_id = self.next_job_id + 1 as u128;
	self.next_job_id += 1;

	let job = distro::JobInfo { name: fitness_func_name.clone(),
				    individual: individual.clone(),
				    job_id: job_id,
	};

	let job_str = serde_json::to_string(&job).unwrap();
	match self.job_queue.put(job_str.as_bytes(), 1, Duration::from_secs(0), Duration::from_secs(120)) {
	    Ok(t) => self.current_jobs.push((job_id, individual)),
	    Err(_) => { println!("Failed to schedule job") },
	};
    }

    pub fn wait(&mut self) -> () {
	// hold off or do w/e till scheduled items are finished.
	while self.current_jobs.len() > 0 {
	    println!("Waiting for jobs to finish: {}", self.current_jobs.len());
	    let mut current_job = self.job_queue.reserve_with_timeout(Duration::from_secs(2));
	    match current_job {
		Ok(mut job_info) => {
		    job_info.delete();
		    // self.job_queue.delete(current_job.id());
		    // todo: pair fitness with the scheduled fitness items.
		    let mut i = 0;
		    let unpacked_result: distro::JobResults = serde_json::from_slice(&job_info.body()).unwrap();
		    for (index, job_r) in self.current_jobs.iter().enumerate() {

			if unpacked_result.job_id == job_r.0 { 
			    i = index;
			}
		    }
		    println!("item is index: {}", i);
		    let mut  queued_job = self.current_jobs.remove(i);
		    queued_job.1.fitness = unpacked_result.fitness;
		},

		Err(_) => {
		    println!("No job results found");
		},
	    }
	}
    }
}

fn evaluate_individual(
    beanstalk: &mut Beanstalkd,
    individual: &mut nn::Network,
    fitness_func: &dyn Fn(&mut nn::Network),
) -> () {
    fitness_func(individual);
}

fn run_ea(
    input_count: u32,
    output_count: u32,
    pop_count: u64,
    iter_count: u64,
    results_folder: String,
    fitness_func: &impl Fn(&mut nn::Network),
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
    fitness_func(&mut individual);

    for _ in 0..pop_count + 1 {
        specific_pop.push(individual.clone());
    }

    for generation in 0..iter_count {
        let gen_folder = format!("{}/{}", results_folder, generation);
        fs::create_dir_all(gen_folder.clone());

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
        let mut average_fit = 0.0;
	let mut total_fitness = 0.0;
        for ind in specific_pop.iter() {
            total_fitness += ind.fitness();
        }
        average_fit = total_fitness / (specific_pop.len() as f64);

        println!("Fitness ({}), ({})", total_fitness, average_fit);

        // generate offsprint from each of the species.
        // the number of offspring depends on the average fitness of the species.
        for spec in species.iter() {
            // add in the champ of the species in.
            offspring.push(spec.champion.unwrap().clone());
            let mut spec_fitness = spec.total_fitness();

            let num_children = num_child_to_make(total_fitness, spec_fitness, pop_count);

            for _child_num in 0..num_children {
                let mut new_child = spec.generate_offspring(&innovation_history).clone();
                new_child.mutate(&mut innovation_history);
                offspring.push(new_child);
            }
        }

	{
	    let mut schedu = Scheduler::new("192.168.1.77", 11300);
	    for off_p in offspring.iter_mut() {
                // fitness_func(&mut new_child);
                // evaluate_individual(&mut new_child, fitness_func);
		schedu.schedule_job(off_p, &"rasteroids".to_string());
	    }

	    schedu.wait();
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

fn server_runner() -> () {
    let mut schedu = Scheduler::new("192.168.1.77", 11300);


    let mut nn = nn::Network::new(16, 3, true);
    let mut indivs: Vec<nn::Network> = Vec::new();
    for i in 0..1000 { 
	indivs.push(nn::Network::new(16, 3, true));
    }

    for p in indivs.iter_mut() {
	schedu.schedule_job(p,
			    &"rasteroids".to_string());
    }

    println!("Waiting for results");
    schedu.wait();

    println!("Results: {}", indivs[0].fitness());
}

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

/// Given the total fitness, species' fitness, and total pop, generate a total number of
/// items
fn num_child_to_make(total_fitness: f64, species_fitness: f64, total_population: u64) -> u64 {
    println!(
        "Total: ({}) Spec: ({}) Pop: ({})",
        total_fitness, species_fitness, total_population
    );
    assert!(total_fitness >= species_fitness);
    ((species_fitness / total_fitness) * total_population as f64) as u64
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
