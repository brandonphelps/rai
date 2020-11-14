use rand::seq::SliceRandom;
#[allow(deprecated)]
use rand::distributions::{Normal, Distribution};
use std::cmp::Reverse;
use std::{thread, time};
use rand::prelude::*;
use prgrs::{Prgrs, Length};

mod hrm;
mod nn;
mod neat;

trait Individual {
    // can this return just a numeric traited instance?
    // post calculated fitness. 
    fn fitness(&self) -> f64;
    fn update_fitness(&mut self) -> ();
    fn print(&self) -> ();
    fn mutate(&mut self) -> ();
    // fn crossover(&self, other: Box<dyn Individual>) -> Box<dyn Individual>;
}

trait Crossover<Rhs=Self> {
    type Output;

    fn crossover(&self, rhs: &Rhs) -> Self::Output;
}

use std::sync::atomic::{AtomicUsize, Ordering};

static SortedIdCount: AtomicUsize = AtomicUsize::new(0);
static SinFIdCount: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
struct SinF {
    pub value: f64,
    pub ident: usize
}


impl SinF {
    // fn crossover(&self, other: &SinF) -> Box<SinF> {
    //     return Box::new(SinF { value: ((self.value + other.value) / 2.0)});
    // }

    fn new(value: f64) -> SinF {
        let old_count = SinFIdCount.fetch_add(1, Ordering::SeqCst);
        SinF{value: value, ident: old_count}
    }
}

impl Crossover for SinF {
    type Output = SinF;
    fn crossover(&self, _rhs: &SinF) -> SinF {
        SinF::new((_rhs.value + self.value) / 2.0)
    }
}

impl Individual for SinF {
    fn update_fitness(&mut self) -> () {

    }

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


#[derive(Debug)]
struct TestNetwork  {
    pub network: nn::Network,
    pub inno_ids: Vec<u64>,
    fitness: f64,
}

static GLOBAL_INNO_ID: u64 = 0;

impl TestNetwork {
    fn new(input_count: u32, output_count: u32) -> TestNetwork {
        let network = nn::Network::new(input_count, output_count, true);

        let mut inno_ids: Vec<u64> = Vec::new();
        for (edge_index, edge) in network.edges.iter().enumerate() {
            inno_ids.push(edge_index as u64);
        }

        return TestNetwork {
            network: network,
            fitness: 0.0,
            inno_ids: inno_ids
        };
    }


    fn mutate_edge(&mut self, edge: usize ) -> () {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < 0.1 {
            self.network.edges[edge].weight = rng.gen::<f64>();
        }
        else {
            let normal = Normal::new(0.0, 0.5);
            let delta = rng.sample::<f64, _>(&normal);
            self.network.edges[edge].weight += delta;
            if self.network.edges[edge].weight  > 1.0 {
                self.network.edges[edge].weight = 1.0;
            }
            else if self.network.edges[edge].weight < -1.0 {
                self.network.edges[edge].weight = -1.0;
            }
        }
    }
}

impl Individual for TestNetwork {

    fn fitness(&self) -> f64 {
        return self.fitness;
    }

    fn update_fitness(&mut self) -> () {
        let mut fitness = 0.0;
        // self.network.pretty_print();
        let mut output = self.network.feed_input(vec![0.0, 0.0]);
        assert_eq!(output.len(), 1);
        fitness += (output[0] - 0.0).powf(2.0);
        output = self.network.feed_input(vec![0.0, 1.0]);
        fitness += (output[0] - 1.0).powf(2.0);
        output = self.network.feed_input(vec![1.0, 0.0]);
        fitness += (output[0] - 1.0).powf(2.0);
        output = self.network.feed_input(vec![1.0, 1.0]);
        fitness += (output[0] - 0.0).powf(2.0);
        fitness /= 4.0;

        if fitness == 0.0 {
            self.fitness = 10000000.0;
        }
        else {
            // fitness -= (self.network.nodes.len() as f64 * 0.1);
            // if fitness < 0.0 {
            //     self.fitness = 0.0000001;
            // }
            // else {
            self.fitness = 1.0 / fitness;
            //}
        }
        println!("Fitness: {:?}", self.fitness);
        thread::sleep(time::Duration::from_millis(1000));
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
        if rng.gen::<f64>() < 0.05 && ! self.network.is_fully_connected() {
            let mut node_one = self.network.random_node();
            let mut node_two = self.network.random_node();
            
            
            while self.network.are_connected(node_one, node_two) ||
                self.network.nodes[node_one].layer == self.network.nodes[node_two].layer {
                    node_one = self.network.random_node();
                    node_two = self.network.random_node();
                }
            self.network.add_connection(node_one, node_two, rng.gen::<f64>(), None);
        }

        // 3% add new node. 
        if rng.gen::<f64>() < 0.03 {
            let edge = self.network.random_non_bias_edge();
            self.network.add_node(edge as usize, rng.gen::<f64>(), rng.gen::<f64>(), None);
        }
    }

    fn print(&self) -> () {

    }
}

impl Crossover for TestNetwork {
    type Output = TestNetwork;

    fn crossover(&self, _rhs: &TestNetwork) -> TestNetwork {
        let mut child_network = nn::Network::new(_rhs.network.input_node_count,
                                                _rhs.network.output_node_count, false);
        
        child_network.layer_count = self.network.layer_count;
        child_network.bias_node_id = self.network.bias_node_id;

        let mut rng = rand::thread_rng();
        for edge in self.network.edges.iter() {
            let mut new_edge = edge.clone();
            if rng.gen::<f64>() < 0.9 {
                new_edge.enabled = true; 
            }
            else {
                new_edge.enabled = false;
            }
            child_network.edges.push(new_edge);
        }

        for node in self.network.nodes.iter() {
            child_network.nodes.push(node.clone());
        }
        
        // todo: finishe
        let inno_ds: Vec<u64> = Vec::new();
        TestNetwork{network: child_network,
                    fitness: 0.0,
                    inno_ids: inno_ds
        }
    }
}


fn do_fitness_func<T: Individual>(individuals: &Vec<T>) -> () {
    for ind in individuals.iter() {
        ind.fitness();
    }
}

fn select_parents<T: Individual>(individuals: &Vec<T>, parent_count: usize) -> Vec<&T> {
    let mut parents: Vec<&T> = Vec::new();

    let mut fitnessSum: f64 = 0.0;
    for _ind in individuals.iter() {
        fitnessSum += _ind.fitness();
    }
    let mut rng = rand::thread_rng();
        // if rng::<f64>::gen() < 0.25 {
        // }

        // let rand_f: Option<&T> = individuals.choose(&mut rand::thread_rng());
        // match rand_f {
        //     None => panic!("None!"),
        //     Some(fd) => {
        //         parents.push(fd)
        //     },
        // };

    for _ind in 1..parent_count  {
        let mut runningSum: f64 = 0.0;
        let rand: f64 = rng.gen_range(0.0, fitnessSum);
        for p in individuals.iter() {
            runningSum += p.fitness();
            if runningSum > rand {
                parents.push(&p);
            }
        }
    }
    return parents;
}

// todo: allow user to specify parent selection algorithm. 
fn generate_offspring<T>(parents: &Vec<&T>, offspring_count: u128) -> Vec<T>
where
    T: Crossover<Output = T> + Individual
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



fn t_main() {
    let population_count = 20;
    let parent_count = 10;
    let offspring_count = 40;
    let mut iteration_count = 0;
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

    for _n in 1..population_count+1 {
        // pop.push(Box::new(SinF::new((n as f64/100.0))));
        let mut random_network = TestNetwork::new(2, 1);
        random_network.update_fitness();
        specific_pop.push(random_network);
    }


    // fitness evaluation

    do_fitness_func(&specific_pop);
    let mut average_history_per_iter: Vec<f64> = Vec::new();

    for i in Prgrs::new(0..max_iter_count, max_iter_count).set_length_move(Length::Proportional(0.5)) {
        let parents = select_parents(&specific_pop, parent_count);
        let mut offspring = generate_offspring(&parents, offspring_count);
        
        do_fitness_func(&offspring);

        for offpin in offspring.iter_mut() {
            offpin.update_fitness();
        }

        // add in the offspring
        specific_pop.append(&mut offspring);
        
        // cull population 
        specific_pop.sort_by_key(|indiv| Reverse((indiv.fitness() * 1000.0) as i128));
        specific_pop.truncate(population_count);

        assert!(specific_pop.len() == population_count);

        iteration_count += 1;

        let mut average_fit = 0.0;
        for pop in specific_pop.iter() {
            average_fit += pop.fitness();
        }

        let str = format!("{}", average_fit / (specific_pop.len() as f64));
        println!("{}", str);
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


fn main() {
    let p = &mut String::from("hello world");
    let mut default_msg = &mut String::from("default message");
    let mut msg = &mut Some(p);

    {
        if let Some(ref m) = msg {
            println!("{}", m);
        }
        
        print_sometimes(&msg);
        modify_sometimes(&mut msg);
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
        let mut innovation_history = neat::InnovationHistory { global_inno_id: (3 * 4),
                                                               conn_history: vec![] };

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
        let mut network = nn::Network::new(num_inputs, num_outputs, true);
        let mut network_two = nn::Network::new(num_inputs, num_outputs, true);
	
        let mut innovation_history = neat::InnovationHistory { global_inno_id: ((num_inputs + 1) * num_outputs) as usize,
                                                               conn_history: vec![] };
        


        assert_eq!(0, neat::Species::get_excess_disjoint(&network.edges, &network_two.edges));

        let mut network_three = nn::Network::new(num_inputs, num_outputs, false);
        assert_eq!(((num_inputs + 1) * num_outputs) as usize, neat::Species::get_excess_disjoint(&network.edges, &network_three.edges));


        assert_eq!(0.0, neat::Species::get_average_weight_diff(&network.edges, &network_two.edges));


        let mut spec = neat::Species::new(0.5, 0.4, 1.2);
        
        spec.set_champion(&network.edges);
        assert!(spec.same_species(&network.edges));
        assert!(spec.same_species(&network_two.edges));
        assert!(!spec.same_species(&network_three.edges));
    }
}
