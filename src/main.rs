#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]
// dono why this is needed for the doc comamnd to work.
// #![feature(intra_doc_pointers)]

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

mod asteroids_individual;

mod utils;
mod distro;
mod lifetime;
mod promise;
mod scheduler;

mod evo_algo;
mod hrm;
mod individual;
mod neat;
mod nn;

mod leven;
mod bana_individ;

use std::collections::HashMap;
use std::time::Instant;

use std::any::Any;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::evo_algo::{run_ea, generic_offspring_gen, species_crossover, ExtractBrain, GAParams};
use crate::nn::{Network};
use crate::asteroids_individual::{AsteroidsPlayer};
use crate::scheduler::{BeanstalkScheduler};

fn log<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;

    match value_any.downcast_ref::<String>() {
        Some(as_string) => {
            println!("String ({}): {}", as_string.len(), as_string);
        }
        None => {
            println!("{:?}", value);
        }
    }
}


#[cfg(not(feature = "gui"))]
fn main() -> std::result::Result<(), String> {
    let ga_params = GAParams {
        pop_size: 400,
        offspring_count: 10,
        generation_count: 100,
        parent_selection_count: 10,
    };

    let mut a_scheduler = BeanstalkScheduler::<AsteroidsPlayer>::new("192.168.0.4", 11300);
    let mut b_scheduler = BeanstalkScheduler::<bana_individ>::new("192.168.0.4", 11300);
    let mut innovation_history = AsteroidsPlayer::new_inno_history();
    
    impl ExtractBrain for AsteroidsPlayer {
        fn get_brain(&self) -> Network {
            self.brain.clone()
        }

	fn set_brain(&mut self, network: Network) {
	    self.brain = network;
	}
    }

    println!("Asteroids");
    run_ea::<AsteroidsPlayer,
	     neat::InnovationHistory,
	     BeanstalkScheduler<AsteroidsPlayer>>(
        &ga_params,
        &mut innovation_history,
	species_crossover,
        &mut a_scheduler,
    );

    run_ea::<bana_individ::BananaIndivid,
	     Option<u8>,
	     BeanstalkScheduler<bana_individ::BananaIndivid>>(
	&ga_params,
	&mut None,
	generic_offspring_gen,
	&mut b_scheduler);
    return Ok(());

    // let _args: Vec<_> = env::args().collect();

    // use chrono::{Datelike, Local, Timelike, Utc};

    // let now = Utc::now();
    // let (is_pm, mut now_hour) = now.hour12();
    // if is_pm {
    //     now_hour += 12;
    // }
    // let folder_time = format!("{}{}{}", now_hour, now.minute(), now.second());

    // use std::process::Command;

    // let output = Command::new("git")
    //     .args(&["rev-parse", "HEAD"])
    //     .output()
    //     .expect("Failed to get git hash");

    // let runner_version = match std::str::from_utf8(&output.stdout[0..6]) {
    //     Ok(v) => v,
    //     Err(_e) => panic!("Failed to get runner version"),
    // };

    // let results_folder = format!("results/asteroids/{}_{}", folder_time, runner_version);
    // println!("Storing results in {}", results_folder);
    // match fs::create_dir_all(results_folder.clone()) {
    //     Err(e) => println!("Failed to create folder: {}", e),
    //     _ => (),
    // }

    // // run_ea(
    // //     input_node_count,
    // //     output_node_count,
    // //     population_count,
    // //     max_iter_count,
    // //     results_folder,
    // // );

    // Ok(())
}

// todo look at this bench mark thing https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust
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
            neat::get_excess_disjoint(&network.edges, &network_two.edges)
        );

        let network_three = nn::Network::new(num_inputs, num_outputs, false);
        assert_eq!(
            ((num_inputs + 1) * num_outputs) as usize,
            neat::get_excess_disjoint(&network.edges, &network_three.edges)
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
            neat::get_excess_disjoint(&network.edges, &network_two.edges)
        );
        assert_eq!(
            0.0,
            neat::Species::get_average_weight_diff(&network.edges, &network_two.edges)
        );

        let network_three = nn::Network::new(num_inputs, num_outputs, false);
        assert_eq!(
            ((num_inputs + 1) * num_outputs) as usize,
            neat::get_excess_disjoint(&network.edges, &network_three.edges)
        );

        network_two.add_node(0, 0.2, 0.4, Some(&mut innovation_history));
        assert_eq!(
            2,
            neat::get_excess_disjoint(&network.edges, &network_two.edges)
        );
        network_two.add_node(5, 0.2, 0.4, Some(&mut innovation_history));
        assert_eq!(
            4,
            neat::get_excess_disjoint(&network.edges, &network_two.edges)
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

}
