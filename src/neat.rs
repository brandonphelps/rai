#![allow(clippy::unused_unit)]
use crate::asteroids;
use crate::evo_algo::{Crossover, Individual};
use crate::nn::{Edge, Network};
use rand::distributions::{Distribution, Normal};
use rand::prelude::*;
use rand::seq::SliceRandom;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::time::{Duration, Instant};
use std::{thread, time};

fn matching_edge(parent2: &Network, inno_id: u64) -> Option<&Edge> {
    for edge in parent2.edges.iter() {
        if edge.inno_id == inno_id {
            return Some(&edge);
        }
    }
    return None;
}

impl Crossover for Network {
    type Output = Network;

    /// Creates a new network from two networks.  
    fn crossover(&self, _rhs: &Network) -> Network {
        let mut child_network = Network::new(_rhs.input_node_count, _rhs.output_node_count, false);

        child_network.layer_count = self.layer_count;
        child_network.bias_node_id = self.bias_node_id;

        let mut rng = rand::thread_rng();
        for edge in self.edges.iter() {
            let parent2_edge_maybe = matching_edge(_rhs, edge.inno_id);
            let mut child_edge_enabled = true;
            // if parent 2 also contains the same edge then determine which to use.
            if let Some(parent2_edge) = parent2_edge_maybe {
                if !edge.enabled || !parent2_edge.enabled && rng.gen::<f64>() < 0.75 {
                    child_edge_enabled = false;
                }
                // determine if edge froms from parent1 or parent2
                let mut new_edge;
                if rng.gen::<f64>() < 0.5 {
                    new_edge = edge.clone();
                    new_edge.enabled = child_edge_enabled;
                } else {
                    new_edge = parent2_edge.clone();
                    new_edge.enabled = child_edge_enabled;
                }
                child_network.edges.push(new_edge);
            } else {
                // disjoint edge from parent 1 and parent 2.
                child_network.edges.push(edge.clone());
            }
        }

        for node in self.nodes.iter() {
            child_network.nodes.push(node.clone());
        }
        return child_network;
    }
}

#[derive(Debug)]
pub struct TestNetwork {
    pub network: Network,
    fitness: f64,
}

impl TestNetwork {
    pub fn new(input_count: u32, output_count: u32) -> TestNetwork {
        let network = Network::new(input_count, output_count, true);

        return TestNetwork {
            network,
            fitness: 0.0,
        };
    }

    pub fn from_network(network: Network) -> TestNetwork {
        return TestNetwork {
            network,
            fitness: 0.0,
        };
    }

    pub fn custom_mutate(&mut self, inno_history: &mut InnovationHistory) -> () {
        let mut rng = rand::thread_rng();
        // 80% chance to mutate edges node.
        if rng.gen::<f64>() < 0.8 {
            for edge_index in 0..self.network.edges.len() {
                self.mutate_edge(edge_index);
            }
        }

        // 5% add new connection
        if rng.gen::<f64>() < 0.05 && !self.network.is_fully_connected() {
            let mut node_one = self.network.random_node();
            let mut node_two = self.network.random_node();

            while self.network.are_connected(node_one, node_two)
                || self.network.nodes[node_one].layer == self.network.nodes[node_two].layer
            {
                node_one = self.network.random_node();
                node_two = self.network.random_node();
            }
            self.network
                .add_connection(node_one, node_two, rng.gen::<f64>(), Some(inno_history));
        }

        // 3% add new node.
        if rng.gen::<f64>() < 0.03 {
            let edge = self.network.random_non_bias_edge();
            self.network.add_node(
                edge as usize,
                rng.gen::<f64>(),
                rng.gen::<f64>(),
                Some(inno_history),
            );
        }
    }

    pub fn mutate_edge(&mut self, edge: usize) -> () {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < 0.1 {
            self.network.edges[edge].weight = rng.gen::<f64>();
        } else {
            let normal = Normal::new(0.0, 0.5);
            let delta = rng.sample::<f64, _>(&normal);
            self.network.edges[edge].weight += delta;
            if self.network.edges[edge].weight > 1.0 {
                self.network.edges[edge].weight = 1.0;
            } else if self.network.edges[edge].weight < -1.0 {
                self.network.edges[edge].weight = -1.0;
            }
        }
    }
}

impl Individual for TestNetwork {
    fn fitness(&self) -> f64 {
        return self.fitness;
    }

    fn update_fitness(&mut self, canvas: &mut Canvas<Window>) -> () {
        let mut _fitness = 0.0;
        // self.network.pretty_print();
        let output = self.network.feed_input(vec![0.0, 0.0]);
        assert_eq!(output.len(), 3);

        let mut game_input = asteroids::GameInput {
            shoot: false,
            thrusters: false,
            rotation: 0.0,
        };

        let mut asteroids_game = asteroids::game_init();

        // vision

        // each item of vision is both a direction and distance to an asteroid.
        // the distance is from the ship, the network will have to figure out that
        // the order of the input is clockwise from north.
        let mut duration = 0;
        let max_turns = 3000;
        for _i in 0..max_turns {
            if output[2] > 0.5 {
                game_input.thrusters = true;
            }

            if output[1] < 0.5 {
                game_input.shoot = true;
            }

            game_input.rotation = output[0];

            asteroids_game = asteroids::game_update(
                &asteroids_game,
                (duration as f64) * 0.01,
                &game_input,
                canvas,
            );
            let start = Instant::now();
            canvas.present();

            if asteroids_game.game_over {
                if asteroids_game.game_over_is_win {
                    self.fitness = 1000000.0;
                } else {
                    self.fitness = (_i as f64 / max_turns as f64) as f64;
                }
                break;
            }

            thread::sleep(Duration::from_millis(10));
            duration = start.elapsed().as_millis();
            game_input.shoot = false;
            game_input.thrusters = false;
        }
    }

    fn mutate(&mut self) -> () {
        let mut rng = rand::thread_rng();
        // 80% chance to mutate edges node.
        if rng.gen::<f64>() < 0.8 {
            for edge_index in 0..self.network.edges.len() {
                self.mutate_edge(edge_index);
            }
        }

        // 5% add new connection
        if rng.gen::<f64>() < 0.05 && !self.network.is_fully_connected() {
            let mut node_one = self.network.random_node();
            let mut node_two = self.network.random_node();

            while self.network.are_connected(node_one, node_two)
                || self.network.nodes[node_one].layer == self.network.nodes[node_two].layer
            {
                node_one = self.network.random_node();
                node_two = self.network.random_node();
            }
            self.network
                .add_connection(node_one, node_two, rng.gen::<f64>(), None);
        }

        // 3% add new node.
        if rng.gen::<f64>() < 0.03 {
            let edge = self.network.random_non_bias_edge();
            self.network
                .add_node(edge as usize, rng.gen::<f64>(), rng.gen::<f64>(), None);
        }
    }

    fn print(&self) -> () {}
}

impl Crossover for TestNetwork {
    type Output = TestNetwork;

    fn crossover(&self, _rhs: &TestNetwork) -> TestNetwork {
        let child_network = self.network.crossover(&_rhs.network);
        TestNetwork {
            network: child_network,
            fitness: 0.0,
        }
    }
}

pub struct Species<'a> {
    excess_coeff: f64,
    weight_diff_coeff: f64,
    compat_threashold: f64,
    pub champion: Option<&'a TestNetwork>,
    pub individuals: Vec<&'a TestNetwork>,
}

impl<'a> Species<'a> {
    pub fn new(excess_coeff: f64, weight_diff_coeff: f64, compat_threashold: f64) -> Species<'a> {
        return Species {
            excess_coeff,
            weight_diff_coeff,
            compat_threashold,
            champion: None,
            individuals: vec![],
        };
    }

    pub fn set_champion(&mut self, new_champ: &'a TestNetwork) -> () {
        self.champion = Some(new_champ);
        self.individuals.push(new_champ);
    }

    pub fn same_species(&self, other: &Vec<Edge>) -> bool {
        let excess_disjoin =
            Species::get_excess_disjoint(&self.champion.unwrap().network.edges, other);
        let average_weight_diff =
            Species::get_average_weight_diff(&self.champion.unwrap().network.edges, other);

        let compat = (self.excess_coeff * excess_disjoin as f64)
            + (self.weight_diff_coeff * average_weight_diff);

        return self.compat_threashold > compat;
    }

    /// returns the average fitness of the individuals within
    /// make sure that all individuals have their update_fitness funcs called before this one.
    pub fn average_fitness(&self) -> f64 {
        // todo: does rust have a fancy functional programming func here?
        let mut avg_fitness = 0.0;
        for ind in self.individuals.iter() {
            avg_fitness += ind.fitness();
        }
        return avg_fitness / self.individuals.len() as f64;
    }

    /// returns the number of excess and disjoint edges.
    /// i.e the number of extra edges and the number of non matching edges.
    pub fn get_excess_disjoint(one: &Vec<Edge>, two: &Vec<Edge>) -> usize {
        let mut matching = 0;
        for edge_one in one.iter() {
            for edge_two in two.iter() {
                if edge_one.inno_id == edge_two.inno_id {
                    matching += 1;
                    break;
                }
            }
        }
        return one.len() + two.len() - 2 * matching;
    }

    pub fn get_average_weight_diff(one: &Vec<Edge>, two: &Vec<Edge>) -> f64 {
        let mut matching = 0;
        let mut total_diff = 0.0;

        for edge_one in one.iter() {
            for edge_two in two.iter() {
                if edge_one.inno_id == edge_two.inno_id {
                    matching += 1;
                    total_diff += (edge_one.weight - edge_two.weight).abs();
                    break;
                }
            }
        }

        if matching == 0 {
            // divide by zero.
            return 100.0; // todo make this an option?
        }
        return total_diff / matching as f64;
    }

    pub fn generate_offspring(&self, _inno_history: &InnovationHistory) -> Network {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < 0.25 {
            return self.individuals[0].network.clone();
        }

        let p_one = &self.individuals.choose(&mut rng).unwrap().network;
        let p_two = &self.individuals.choose(&mut rng).unwrap().network;
        return p_one.crossover(&p_two);
    }
}

#[derive(Debug)]
pub struct InnovationHistory {
    pub global_inno_id: usize,
    pub conn_history: Vec<ConnHistory>,
}

impl InnovationHistory {
    pub fn get_inno_number(
        &mut self,
        network_inno_ids: &Vec<u64>,
        from_node: usize,
        to_node: usize,
    ) -> usize {
        let mut is_new = true;
        // todo: change zero to next conn number.
        let mut connect_inno_num = self.global_inno_id;
        self.global_inno_id += 1;
        for conn_history in self.conn_history.iter() {
            if conn_history.matches(network_inno_ids, from_node, to_node) {
                is_new = false;
                connect_inno_num = conn_history.inno_number;
                break;
            }
        }

        if is_new {
            let mut new_inno_nums = Vec::<u64>::new();
            for edge in network_inno_ids.iter() {
                new_inno_nums.push(edge.clone());
            }

            self.conn_history.push(ConnHistory::new(
                from_node,
                to_node,
                connect_inno_num,
                new_inno_nums,
            ));
        }

        return connect_inno_num;
    }
}

#[derive(Debug)]
pub struct ConnHistory {
    from_node: usize,
    to_node: usize,
    inno_number: usize,
    inno_numbers: Vec<u64>,
}

impl ConnHistory {
    pub fn new(
        from_node: usize,
        to_node: usize,
        inno_number: usize,
        inno_numbers: Vec<u64>,
    ) -> ConnHistory {
        return ConnHistory {
            from_node,
            to_node,
            inno_number,
            inno_numbers,
        };
    }

    /// inserts and returns a new inno_id or if already existing returns current one.
    pub fn matches(&self, network_inno_ids: &Vec<u64>, from_node: usize, to_node: usize) -> bool {
        // ConnHistory must have the same number of numbers as edges
        if network_inno_ids.len() == self.inno_numbers.len() {
            if from_node == self.from_node && to_node == self.to_node {
                for inno_index in 0..network_inno_ids.len() {
                    let net_inno_id = &network_inno_ids[inno_index];
                    if !self
                        .inno_numbers
                        .iter()
                        .any(|inno_id| net_inno_id == inno_id)
                    {
                        return false;
                    }
                }
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_matches() {
        let mut network: Vec<u64> = Vec::new();
        for edge_count in 0..12 {
            network.push(edge_count);
        }
        let global_inno_id = 2;

        let mut innovation_history = InnovationHistory {
            global_inno_id: global_inno_id,
            conn_history: vec![],
        };

        let new_inno_num = innovation_history.get_inno_number(&network, 0, 2);
        assert_eq!(new_inno_num, 2);
        let conn_history = &innovation_history.conn_history[0];

        println!("{:#?}", conn_history);
        println!("{:#?}", innovation_history);

        assert!(conn_history.matches(&network, 0, 2));

        // new connection that hasn't been seen before.
        let new_inno_num = innovation_history.get_inno_number(&network, 0, 20);
        assert_eq!(new_inno_num, global_inno_id + 1);
    }
}
