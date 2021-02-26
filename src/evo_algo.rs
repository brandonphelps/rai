#![allow(clippy::unused_unit)]
#![allow(dead_code)]

use rand::prelude::*;
use rand::seq::SliceRandom;

pub trait Individual {
    // can this return just a numeric traited instance?
    // post calculated fitness.
    fn fitness(&self) -> f64;
    fn update_fitness(&mut self) -> ();
    fn print(&self) -> ();
    fn mutate(&mut self) -> ();
    // fn crossover(&self, other: Box<dyn Individual>) -> Box<dyn Individual>;
}

pub trait Crossover<Rhs = Self> {
    type Output;

    fn crossover(&self, rhs: &Rhs) -> Self::Output;
}

pub trait RandomDefault: Default {
    // empty
}

pub trait Fitness {
    fn fitness(&self) -> f32;
}

pub trait IndividualTrait: Clone {
    fn fitness(&self) -> f32;
    fn default() -> Self;
    fn crossover(&self, other: &Self) -> Self;
    fn mutate(&self) -> Self;
}

/// @brief container class for the various parameters.
pub struct GAParams {
    pub pop_size: usize,
    
    /// Number of generations to run the simulation for.
    pub generation_count: usize,
    pub parent_selection_count: usize,
}

/// This trait provides a global storage mechanism for items you'd like to have
pub trait GAStorage {
    // empty
}

/// @brief container instance of all the data associated to the
/// GA at a given point in time. Holds data that is used for each of the call backs
/// Each callback should use the functions provided to update the instance.
/// one should provide the individual container item.
/// individuals are lifetimed to the GAState and should provide a Default func,
// todo: use the where stuff to constrain the individualT as appropriate.
// todo; can IndividualT be removed and replaced with IndividualTrait?
pub struct GAFunctors<IndividualT, Storage>
where
    IndividualT: IndividualTrait,
{
    // on_start: fn(&mut Self, &mut Vec<IndividualT>) -> f32,
    // on_fitness: fn(&mut Self, &IndividualT) -> f32,
    on_start: fn(params: &GAParams, &mut Vec<IndividualT>),

    // do we even need this at all? if IndividualT is met or constrained to contain
    //  a fitness func then that is there. 
    // do we need GAParams here?
    on_fitness: fn(params: &GAParams, &IndividualT) -> f32,
    on_parents: fn(&mut Storage, parents: &Vec<&IndividualT>) -> f32,
    on_crossover: fn(&mut Storage, parents: &Vec<&IndividualT>) -> Vec<IndividualT>,
    on_mutation: fn(&mut Storage, newly_created_individuals: &Vec<&IndividualT>) -> f32,
    // on_mutation: fn(&mut Storage, newly_created_individuals, offspring_mutation_size);
    // on_generation: fn(&mut Storage);

    // on_stop(last_pop_fitness);
}

fn empty_start<T>(params: &GAParams, indi: &mut Vec<T>)
where
    T: IndividualTrait,
{
    for _i in 0..params.pop_size {
	indi.push(T::default());
    }

    assert_eq!(indi.len(), params.pop_size);
}


fn empty_fitness<T>(params: &GAParams, individual: &T) -> f32
where
    T: IndividualTrait,
{
    individual.fitness()
}

fn empty_parents<T, S>(_storage: &mut S, _parents: &Vec<&T>) -> f32 {
    0.0
}

fn empty_crossover<T, S>(_storage: &mut S, _parents: &Vec<&T>) -> Vec<T>
where
    T: IndividualTrait
{
    let mut new_generation = Vec::<T>::new();

    let mut rng = rand::thread_rng();

    if rng.gen::<f64>() < 0.25 {
	let p = *_parents.choose(&mut rng).unwrap();
	new_generation.push(p.clone());
    }

    let p_one = _parents.choose(&mut rng).unwrap();
    let p_two = _parents.choose(&mut rng).unwrap();

    new_generation.push(p_one.crossover(&p_two));

    return new_generation;
}

fn empty_mutation<T, S>(_storage: &mut S, _new_individuals: &Vec<&T>) -> f32 {
    0.0
}

impl<IndividualT, Storage> GAFunctors<IndividualT, Storage>
where
    IndividualT: IndividualTrait,
{
    pub fn new() -> Self {
        GAFunctors {
            on_start: empty_start,
            on_fitness: empty_fitness,
            on_parents: empty_parents,
	    on_crossover: empty_crossover,
	    on_mutation: empty_mutation,
        }
    }
}

pub fn run_ea_simple<T>(ga_params: &GAParams,
			on_start: impl Fn()) -> ()
where
    T: IndividualTrait
{
    
}

// todo: need some generic global storage for some EA systems.
pub fn run_ea<T, GAStorage>(
    ga_params: &GAParams,
    ga_storage: &mut GAStorage,
    ga_functors: &mut GAFunctors<T, GAStorage>,
) -> ()
where
    T: IndividualTrait,
{
    let mut individuals = Vec::<T>::new();

    // startup & initialization
    (ga_functors.on_start)(&ga_params, &mut individuals);
    // produce generation (each individual should provide #[default]
    // calculate fitness

    for _current_gen in 0..ga_params.generation_count {
        let mut fitness_vec = Vec::<f32>::new();
        for individual in individuals.iter() {
            let tmp = (ga_functors.on_fitness)(&ga_params, &individual);
            fitness_vec.push(tmp);
        }

        // parent selection algorithm.

        let mut parents = Vec::<&T>::new();

        // todo:replace with actual parent selection algorithm.
        // allow for parent selection process to be selected from list type.
        for individual in 0..10 {
            parents.push(&individuals[individual]);
        }

        (ga_functors.on_parents)(ga_storage, &parents);

	(ga_functors.on_crossover)(ga_storage, &parents);
    }
    // }
    // do cross over
    // on_crossover
    // do mutation
    // on_mutation()
    // on_generation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct test_individual {
        w1: f32,
        w2: f32,
        w3: f32,
        w4: f32,
        w5: f32,
        w6: f32,

        x1: f32,
        x2: f32,
        x3: f32,
        x4: f32,
        x5: f32,
        x6: f32,
    }

    impl IndividualTrait for test_individual {
        fn default() -> Self {
            println!("Generating default individual");
            test_individual {
                w1: 3.0,
                w2: 1.0,
                w3: 5.0,
                w4: 5.0,
                w5: 1.0,
                w6: 3.0,

		// todo: make these random.
                x1: 0.0,
                x2: 0.0,
                x3: 0.0,
                x4: 0.0,
                x5: 0.0,
                x6: 0.0,
            }
        }

        fn fitness(&self) -> f32 {
            self.w1 * self.x1
                + self.w2 * self.x2
                + self.w3 * self.x3
                + self.w4 * self.x4
                + self.w5 * self.x5
                + self.w6 * self.x6
	}

	fn mutate(&self) -> Self {
	    let mut f = self.clone();
	    f.x1 += 0.02;
	    f.x2 -= 0.1;
	    f.x3 += 0.20;
	    f.x4 += 0.21;
	    f.x5 += 0.30;
	    f.x6 += 0.32;
	    return f;
	}

	fn crossover(&self, other: &Self) -> Self {
	    let mut f = self.clone();
	    f.x1 = (other.x1 + self.x1 ) / 2.0;
	    f.x2 = (other.x2 + self.x2 ) / 2.0;
	    f.x3 = (other.x3 + self.x3 ) / 2.0;
	    f.x3 = (other.x3 + self.x3 ) / 2.0;
	    f.x5 = (other.x5 + self.x5 ) / 2.0;
	    f.x6 = (other.x6 + self.x6 ) / 2.0;
	    return f;
	}
    }

    struct CustomStorage {
        pub gen_count: u32,

	// initial value of weight two. 
	pub init_w2: f32,
    }

    impl GAStorage for CustomStorage {}

    // this seems kinda funky, but note that on_parents is of <T, S> but since we
    // specify the S here as CustomStorage Concrete class we all g. 
    fn test_on_parents<T>(storage: &mut CustomStorage, parents: &Vec<&T>) -> f32
    where
        T: IndividualTrait,
    {
        storage.gen_count += 1;
        return 0.0;
    }

    #[test]
    fn test_playground() {
        let mut ga_functors = GAFunctors::<test_individual, CustomStorage>::new();
        let ga_params = GAParams {
            pop_size: 10,
            generation_count: 10,
            parent_selection_count: 2,
        };
        let mut storage = CustomStorage { gen_count: 0, init_w2: 3.0 };
        ga_functors.on_parents = test_on_parents;
        run_ea::<test_individual, CustomStorage>(&ga_params,
						 &mut storage,
						 &mut ga_functors);

	println!("Gen Count: {}", storage.gen_count);

        assert!(false);
    }
}
