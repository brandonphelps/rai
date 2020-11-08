use rand::seq::SliceRandom;
use std::cmp::Reverse;
use rand::prelude::*;

mod hrm;
mod nn;

trait Individual {
    // can this return just a numeric traited instance?
    // post calculated fitness. 
    fn fitness(&self) -> u128;
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

    fn fitness(&self) -> u128 {
        let _p = self.value * self.value.sin().powf(2.0);
        return ((_p + 100.0) * 1000.0) as u128;
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
    for _ind in 1..parent_count  {
        let rand_f: Option<&T> = individuals.choose(&mut rand::thread_rng());
        match rand_f {
            None => panic!("None!"),
            Some(fd) => {
                parents.push(fd)
            },
        };
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

#[derive(Debug)]
struct TestNetwork  {
    pub network: nn::Network,
    fitness: u128,
}

impl TestNetwork {
    fn new(input_count: u32, output_count: u32) -> TestNetwork {
        let mut network = nn::Network::new(input_count, output_count, true);
        return TestNetwork {
            network: network,
            fitness: 0
        };
    }
}

impl Individual for TestNetwork {

    fn fitness(&self) -> u128 {
        return self.fitness;
    }

    fn update_fitness(&mut self) -> () {
        let mut fitness = 0.0;
        // self.network.pretty_print();
        let mut output = self.network.feed_input(vec![0.0, 0.0]);
        assert_eq!(output.len(), 1);
        // println!("Input: 0, 0: {:?}", output[0]);
        fitness += (output[0] - 0.0).powf(2.0);
        //println!("Fitness: {:?}", fitness);
        output = self.network.feed_input(vec![0.0, 1.0]);
        // println!("Input: 0, 1: {:?}", output);
        fitness += (output[0] - 1.0).powf(2.0);
        // println!("Fitness: {:?}", fitness);
        output = self.network.feed_input(vec![1.0, 0.0]);
        println!("Input: 1, 0: {:?}", output);
        fitness += (output[0] - 1.0).powf(2.0);
        println!("Fitness: {:?}", fitness);
        fitness = fitness / 3.0;
        // println!("Final fitness: {:?}", fitness);
        if fitness != 0.0 {
            self.fitness = (1.0 / fitness) as u128;
        }
        else {
            self.fitness = 10000;
        }
    }

    fn mutate(&mut self) -> () {
        let mut rng = rand::thread_rng();
        // 90% chance to add another node. 
        if rng.gen::<f64>() < 0.9 {
            let edge = self.network.random_non_bias_edge();
            println!("Add node");
            self.network.add_node(edge as usize, 0.4, 0.5);
        }

        for f_edge in self.network.edges.iter_mut() {
            if f_edge.enabled && f_edge.from_node != self.network.bias_node_id {
                // chance to change weight of an edge
                if rng.gen::<f64>() < 0.3 {
                    if rng.gen::<f64>() < 0.5 {
                        println!("Weight gain");
                        f_edge.weight += 4.0;
                    }
                    else {
                        println!("Weight lose");
                        f_edge.weight -= 4.0;
                    }
                }
            }
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
        
        TestNetwork{network: child_network, fitness: 0}
    }
}

#[cfg(test)]
 mod tests {
     use super::*;

 }


fn main() {
    let population_count = 300;
    let parent_count = 20;
    let offspring_count = 30;
    let mut iteration_count = 0;
    let max_iter_count = 100;
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

    while iteration_count < max_iter_count {
        // Select Parents. 
        let parents = select_parents(&specific_pop, parent_count);
        let mut offspring = generate_offspring(&parents, offspring_count);
        
        do_fitness_func(&offspring);

        for offpin in offspring.iter_mut() {
            offpin.update_fitness();
            println!("{}", offpin.fitness());
        }

        // add in the offspring
        specific_pop.append(&mut offspring);
        
        // cull population 
        specific_pop.sort_by_key(|indiv| Reverse(indiv.fitness()));
        specific_pop.truncate(population_count);

        assert!(specific_pop.len() == population_count);

        iteration_count += 1;
    }

    // generate fitness values.

    specific_pop.sort_by_key(|indiv| Reverse(indiv.fitness()));
    println!("Top Ten");
    for offp in 1..10 {
        println!("{} {:?} {}", offp, specific_pop[offp], specific_pop[offp].fitness());
    }

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

fn t_main() {
    let mut network = TestNetwork::new(2, 1);

    network.update_fitness();
    for i in 0..4 {
        network.network.pretty_print();
        network.mutate();
        network.update_fitness();
    }
}
