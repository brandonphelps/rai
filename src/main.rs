#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]
use prgrs::{Length, Prgrs};
use rand::distributions::{Distribution, Normal};
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::cmp::Reverse;
use std::{thread, time};

// are these important?
mod hrm;
mod neat;
mod nn;
mod evo_algo;

use evo_algo::{Individual, Crossover};
use neat::{TestNetwork};

use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(non_upper_case_globals)]
static SortedIdCount: AtomicUsize = AtomicUsize::new(0);
static SinFIdCount: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
struct SinF {
    pub value: f64,
    pub ident: usize,
}

impl SinF {
    // fn crossover(&self, other: &SinF) -> Box<SinF> {
    //     return Box::new(SinF { value: ((self.value + other.value) / 2.0)});
    // }

    fn new(value: f64) -> SinF {
        let old_count = SinFIdCount.fetch_add(1, Ordering::SeqCst);
        SinF {
            value: value,
            ident: old_count,
        }
    }
}

impl Crossover for SinF {
    type Output = SinF;
    fn crossover(&self, _rhs: &SinF) -> SinF {
        SinF::new((_rhs.value + self.value) / 2.0)
    }
}

impl Individual for SinF {
    fn update_fitness(&mut self) -> () {}

    fn fitness(&self) -> f64 {
        let _p = self.value * self.value.sin().powf(2.0);
        return (_p + 100.0) * 1000.0;
    }

    fn mutate(&mut self) -> () {
        self.value = 0.01;
    }

    fn print(&self) -> () {
        print!("{:?}", self)
    }
}


fn do_fitness_func<T: Individual>(individuals: &Vec<T>) -> () {
    for ind in individuals.iter() {
        ind.fitness();
    }
}

fn select_parents<T: Individual>(individuals: &Vec<T>, parent_count: usize) -> Vec<&T> {
    let mut parents: Vec<&T> = Vec::new();

    let mut fitness_sum: f64 = 0.0;
    for _ind in individuals.iter() {
        fitness_sum += _ind.fitness();
    }
    let mut rng = rand::thread_rng();

    for _ind in 1..parent_count {
        let mut running_sum: f64 = 0.0;
        let rand: f64 = rng.gen_range(0.0, fitness_sum);
        for p in individuals.iter() {
            running_sum += p.fitness();
            if running_sum > rand {
                parents.push(&p);
            }
        }
    }
    return parents;
}

// todo: allow user to specify parent selection algorithm.
fn generate_offspring<T>(parents: &Vec<&T>, offspring_count: u128) -> Vec<T>
where
    T: Crossover<Output = T> + Individual,
{
    let mut offspring: Vec<T> = Vec::new();

    // breed offspring / mutate
    let parent_one = match parents.choose(&mut rand::thread_rng()) {
        None => panic!("None!"),
        Some(fd) => fd,
    };

    let parent_two = match parents.choose(&mut rand::thread_rng()) {
        None => panic!("None!"),
        Some(fd) => fd,
    };

    for _offp in 1..offspring_count {
        let mut child = parent_one.crossover(parent_two);
        child.mutate();
        offspring.push(child);
    }

    return offspring;
}

// fn run_dah_simulation<T>(initial_pop: Vec<T>, pop_count: u64, parent_count: u64, offspring_count: u64, iter_count: u64)
// where
//     T: Crossover<Output = T> + Individual
// {

// }

fn main() {
    let population_count = 200;
    let mut _iteration_count = 0;
    let max_iter_count = 10000;
    // let mut specific_pop: Vec<SinF> = Vec::new();
    let mut specific_pop: Vec<TestNetwork> = Vec::new();

    // generate random populateion
    // for n in 1..population_count+1 {
    //     // pop.push(Box::new(SinF::new((n as f64/100.0))));
    //     let mut sinfff = SinF::new(n as f64/100.0);
    //     sinfff.update_fitness();
    //     specific_pop.push(sinfff);
    // }

    for _n in 1..population_count + 1 {
        // pop.push(Box::new(SinF::new((n as f64/100.0))));
        let mut random_network = TestNetwork::new(2, 1);
        random_network.update_fitness();
        specific_pop.push(random_network);
    }

    // fitness evaluation
    let mut innovation_history = neat::InnovationHistory {
        // todo mark 3 and 1 as input + (1)bias * output
        global_inno_id: (3 * 1),
        conn_history: vec![],
    };

    let mut average_history_per_iter: Vec<f64> = Vec::new();

    for _i in
        Prgrs::new(0..max_iter_count, max_iter_count).set_length_move(Length::Proportional(0.5))
    {
        let mut species: Vec<neat::Species> = Vec::new();
        // why can't this forloop be outside this forloop? something
        // about the specific_pop updating is mutable borrow after an immutable barrow on something?
        for test_n in specific_pop.iter() {
            let mut found_spec = false;
            for spec in species.iter_mut() {
                if spec.same_species(&test_n.network.edges) {
                    spec.individuals.push(&test_n);
                    found_spec = true;
                }
            }
            if ! found_spec {
                let mut new_spec = neat::Species::new(1.5, 0.8, 4.0);
                new_spec.set_champion(&test_n);
                species.push(new_spec);
            }
        }

        let mut offspring: Vec<TestNetwork> = Vec::new();
        
        for spec in species.iter() {
            // add in the champ of the species in. 
            offspring.push(TestNetwork::from_network(spec.champion.unwrap().network.clone()));
            for _child_num in 0..10 {
                let mut new_child = TestNetwork::from_network(spec.generate_offspring(&innovation_history));
                new_child.custom_mutate(&mut innovation_history);
                offspring.push(new_child);
            }
        }
        
        let species_count = species.len();
        species.clear();

        // do_fitness_func(&offspring);
        for offpin in offspring.iter_mut() {
            offpin.update_fitness();
        }

        // add in the offspring
        specific_pop.append(&mut offspring);

        // // cull population
        specific_pop.sort_by_key(|indiv| Reverse((indiv.fitness() * 1000.0) as i128));
        specific_pop.truncate(population_count);

        assert!(specific_pop.len() == population_count);

        let mut average_fit = 0.0;
        for pop in specific_pop.iter() {
            average_fit += pop.fitness();
        }
        
        println!("Species({}) average fitness {}", species_count, average_fit / (specific_pop.len() as f64));
        average_history_per_iter.push(average_fit / (specific_pop.len() as f64));
    }

    // generate fitness values.

    specific_pop.sort_by_key(|indiv| Reverse((indiv.fitness() * 1000.0) as i128));
    let top = &mut specific_pop[0].network;

    println!("Results");
    println!("{:#?}", top);
    println!("{:?}", top.feed_input(vec![0.0, 0.0]));
    println!("{:?}", top.feed_input(vec![0.0, 1.0]));
    println!("{:?}", top.feed_input(vec![1.0, 0.0]));
    println!("{:?}", top.feed_input(vec![1.0, 1.0]));

    // println!("{:?}", top.network.feed_input(vec![0.0, 1.0]));
    // println!("{:?}", top.network.feed_input(vec![1.0, 0.0]));
    // println!("{:?}", top.network.feed_input(vec![1.0, 1.0]));

    // let j = hrm::Program::new();

    // let rand_t: Option<&(u32, u32)> = results.choose(&mut rand::thread_rng());
    // let newSinF = match rand_t {
    //     None => panic!("None"),
    //     Some(T) => {
    //         match rand_f {
    //             None => panic!("No idea double None"),
    //             Some(F) => {
    //                 specific_pop[F.1 as usize].crossover(&specific_pop[T.1 as usize])
    //             },
    //         }
    //     }
    // };

    // println!("New sinze F: {:?}", newSinF);
}

// todo look at this bench amrk thing https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust

fn print_sometimes(ref value: &Option<&mut String>) {
    if let Some(ref m) = value {
        println!("{}", m);
    }
    if let Some(m) = &value {
        println!("{}", *m);
    }
}

fn modify_sometimes(value: &mut Option<&mut String>) {
    if let Some(ref mut m) = value {
        m.push(' ');
        m.push('F');
        m.push('O');
        m.push('O');

        println!("{}", m);
    }
}

fn t_main() {
    let p = &mut String::from("hello world");
    let _default_msg = &mut String::from("default message");
    let mut msg = &mut Some(p);

    {
        if let Some(ref m) = msg {
            println!("{}", m);
        }

        print_sometimes(&msg);
        modify_sometimes(&mut msg);
        print_sometimes(&msg);
    }

    if let Some(ref m) = msg {
        println!("{}", m);
    }

    
    // let unwrapped_msg = msg.unwrap_or(default_msg);
    // println!("{}", unwrapped_msg);
}

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

        spec.set_champion(&network.edges);
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

        spec.set_champion(&network.edges);
        assert!(spec.same_species(&network.edges));
        assert!(!spec.same_species(&network_two.edges));
        assert!(!spec.same_species(&network_three.edges));
    }
}
