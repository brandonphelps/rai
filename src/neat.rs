#![allow(clippy::unused_unit)]
#![allow(dead_code)]
use crate::nn::{Edge, Network};
use rand::prelude::*;
use rand::seq::SliceRandom;

pub struct Species<'a> {
    excess_coeff: f64,
    weight_diff_coeff: f64,
    compat_threashold: f64,
    pub champion: Option<&'a Network>,
    individuals: Vec<&'a Network>,
}

/// Given a vector of individuals, return a vector of species, where the individuals
/// are divided into species based on how similar they are.
pub fn speciate(population: &Vec<Network>) -> Vec<Species> {
    let mut species: Vec<Species> = Vec::new();

    for test_n in population.iter() {
        let mut found_spec = false;

        for spec in species.iter_mut() {
            if spec.same_species(&test_n.edges) {
                spec.individuals.push(&test_n);
                found_spec = true;
            }
        }

        if !found_spec {
            // todo: allow the params to be passed in or something.
            let mut new_spec = Species::new(1.0, 0.8, 4.0);
            new_spec.set_champion(&test_n);
            species.push(new_spec);
        }
    }

    return species;
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

    pub fn set_champion(&mut self, new_champ: &'a Network) -> () {
        self.champion = Some(new_champ);
        self.individuals.push(new_champ);
    }

    pub fn same_species(&self, other: &Vec<Edge>) -> bool {
        let excess_disjoin = Species::get_excess_disjoint(&self.champion.unwrap().edges, other);
        let average_weight_diff =
            Species::get_average_weight_diff(&self.champion.unwrap().edges, other);

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

    pub fn total_fitness(&self) -> f64 {
        let mut fitness = 0.0;
        for ind in self.individuals.iter() {
            fitness += ind.fitness();
        }
        return fitness;
    }

    /// returns the number of excess and disjoint edges.
    /// i.e the number of extra edges and the number of non matching edges.
    // todo: move outside the species impl?
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
            // todo: have this randomly choose an individual.
            let p = *self.individuals.choose(&mut rng).unwrap();
            return p.clone();
        }

        let p_one = &self.individuals.choose(&mut rng).unwrap();
        let p_two = &self.individuals.choose(&mut rng).unwrap();
        // todo: put mutation call here?
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
    use crate::nn::{node_per_layer, Edge, Network};    

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

    #[test]
    fn test_excess_disjoin() {
        let edge_one: Vec<Edge> = Vec::new();
        let edge_two: Vec<Edge> = Vec::new();

        assert_eq!(Species::get_excess_disjoint(&edge_one, &edge_two), 0);
    }

    #[test]
    fn test_offspring_generate() {
        let mut species = Species::new(1.5, 2.0, 0.4);

        let network_one = Network::new(3, 2, true);
        let network_two = Network::new(3, 2, true);

        assert_eq!(node_per_layer(&network_one, 0).unwrap(), 4);
        assert_eq!(node_per_layer(&network_one, 1).unwrap(), 2);

        assert_eq!(node_per_layer(&network_two, 0).unwrap(), 4);
        assert_eq!(node_per_layer(&network_two, 1).unwrap(), 2);

        species.individuals.push(&network_one);
        species.individuals.push(&network_two);

        let inno_history = InnovationHistory {
            global_inno_id: (3 * 2) as usize,
            conn_history: vec![],
        };

        for i in 0..100 {
            let offspring = species.generate_offspring(&inno_history);
            assert_eq!(node_per_layer(&offspring, 0).unwrap(), 4);
            assert_eq!(node_per_layer(&offspring, 1).unwrap(), 2);
        }
    }
}
