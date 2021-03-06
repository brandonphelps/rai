#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::nn::Network;

use std::collections::HashMap;

use crate::asteroids_individual::asteroids_fitness;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobInfo {
    pub name: String,
    pub individual: Network,
    pub job_id: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResults {
    pub job_id: u128,
    pub fitness: f64,
}

#[allow(dead_code)]
pub struct EaFuncMap {
    func_map: HashMap<String, fn(&Network) -> f64>,
}

#[cfg(not(feature = "gui"))]
impl EaFuncMap {
    pub fn new() -> Self {
	let mut hash_map: HashMap<String, fn(&Network) -> f64> = HashMap::new();

	hash_map.insert(String::from("rasteroids"), asteroids_fitness);

	EaFuncMap {
	    func_map: hash_map,
	}
    }
    
    #[allow(dead_code)]
    pub fn do_func(func_name: &String, indi: &mut Network) -> () {
        if func_name.as_str() == "rasteroids" {
            asteroids_fitness(indi);
        }
    }

    pub fn run_fitness(&self, func_name: &String, indi: &Network) -> f64 {
	(self.func_map.get(func_name).unwrap())(indi)
    }
}

