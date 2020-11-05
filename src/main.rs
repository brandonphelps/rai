use rand::seq::SliceRandom;

trait Individual {
    // can this return just a numeric traited instance?
    // post calculated fitness. 
    fn fitness(&self) -> u32;
    fn print(&self) -> ();
    fn mutate(&self) -> Box<dyn Individual>;
    // fn crossover(&self, other: Box<dyn Individual>) -> Box<dyn Individual>;
}

trait Crossover<Rhs=Self> {
    type Output;

    fn crossover(&self, rhs: &Box<Rhs>) -> Self::Output;
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
    fn crossover(&self, _rhs: &Box<SinF>) -> SinF {
        println!("Cross over for SinF");
        SinF::new((_rhs.value + self.value) / 2.0)
    }
}

impl Individual for SinF {
    fn fitness(&self) -> u32{
        let _p = self.value * self.value.sin().powf(2.0);

        // todo: could fitnes rely on other members of the pop?
        // todo: maybe do this as a post process? 
        return ((_p + 100.0) * 1000.0) as u32;
    }

    fn mutate(&self) -> Box<dyn Individual> {
        return Box::new(SinF::new(self.value + 0.01));
    }
    
    fn print(&self) -> () {
        print!("{:?}", self)
    }
}

fn get_mah_fitness<T: Individual>(ind: T) {
    ind.fitness();
}

fn gen_new_pop<T: Crossover>(individuals: Vec<Box<T>>) -> Vec<T> {
    let mut results: Vec<T> = Vec::new();

    for item in individuals.iter() {
    }
    return results;
}


fn do_fitness_func<T: Individual>(individuals: &Vec<T>) -> () {
    for ind in individuals.iter() {
        ind.fitness();
    }
}

fn select_parents<T: Individual>(individuals: &Vec<T>, parent_count: usize) -> Vec<&T> {
    let mut parents: Vec<&T> = Vec::new(); 
    for ind in 1..parent_count  {
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


fn main() {
    let mut specific_pop: Vec<SinF> = Vec::new();
    let mut pop: Vec<Box<dyn Individual>> = Vec::new();

    // fitness , index
    let mut results : Vec<(u32, u32)> = Vec::new();

    // generate random populateion
    for n in 1..301 {
        pop.push(Box::new(SinF::new((n as f64/100.0))));
        specific_pop.push(SinF::new((n as f64/100.0)));
    }

    // fitness evaluation

    do_fitness_func(&specific_pop);
    let mut iteration_count = 0;
    let max_iter_count = 100;

    while iteration_count < max_iter_count {

        // Select Parents. 





        iteration_count += 1;
    }



    let mut c = 0;
    for n in pop.iter() {
        results.push((n.fitness(), c));
        c = c + 1;
    }

    // generate fitness values.



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
