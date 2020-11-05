use rand::seq::SliceRandom;
use std::cmp::Reverse;

trait Individual {
    // can this return just a numeric traited instance?
    // post calculated fitness. 
    fn fitness(&self) -> u128;
    fn print(&self) -> ();
    fn mutate(&mut self) -> ();
    // fn crossover(&self, other: Box<dyn Individual>) -> Box<dyn Individual>;
}

trait Crossover<Rhs=Self> {
    type Output;

    fn crossover(&self, rhs: &Rhs) -> Self::Output;
}

use std::sync::atomic::{AtomicUsize, Ordering};

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
            Some(FD) => {
                parents.push(FD)
            },
        };
    }
    return parents;
}

// todo: allow user to specify parent selection algorithm. 
fn generate_offspring<T>(parents: &Vec<T>, offspring_count: u128) -> Vec<T>
where
    T: Crossover<Output = T> + Individual
{
    let mut offspring: Vec<T> = Vec::new();

    // breed offspring / mutate
    let parent_one = match parents.choose(&mut rand::thread_rng()) {
        None => panic!("None!"),
        Some(FD) => FD,
    };

    let parent_two = match parents.choose(&mut rand::thread_rng()) {
        None => panic!("None!"),
        Some(FD) => FD,
    };

    for offp in 1..offspring_count {
        let mut child = parent_one.crossover(parent_two);
        child.mutate();
        offspring.push(child);
    }

    return offspring;
}

fn main() {
    let population_count = 300;
    let parent_count = 20;
    let offspring_count = 30;
    let mut iteration_count = 0;
    let max_iter_count = 100;
    let mut specific_pop: Vec<SinF> = Vec::new();

    // generate random populateion
    for n in 1..population_count+1 {
        // pop.push(Box::new(SinF::new((n as f64/100.0))));
        specific_pop.push(SinF::new(n as f64/100.0));
    }

    // fitness evaluation

    do_fitness_func(&specific_pop);

    while iteration_count < max_iter_count {
        // Select Parents. 
        let parents = select_parents(&specific_pop, parent_count);

        // let mut offspring = generate_offspring(&parents, offspring_count);
        let mut offspring: Vec<SinF> = Vec::new();

        // breed offspring / mutate
        let parent_one = match parents.choose(&mut rand::thread_rng()) {
            None => panic!("None!"),
            Some(FD) => FD,
        };

        let parent_two = match parents.choose(&mut rand::thread_rng()) {
            None => panic!("None!"),
            Some(FD) => FD,
        };

        for offp in 1..offspring_count {
            let mut child = parent_one.crossover(parent_two);
            child.mutate();
            offspring.push(child);
        }

        do_fitness_func(&offspring);

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
