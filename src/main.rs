#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]
use prgrs::{Length, Prgrs};
use rand::distributions::{Distribution, Normal};
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::cmp::Reverse;
use std::{thread, time};

mod asteroids;
mod collision;
mod evo_algo;
mod hrm;
mod neat;
mod nn;

use evo_algo::{Crossover, Individual};
use neat::TestNetwork;
use std::time::{Duration, Instant};

use std::sync::atomic::{AtomicUsize, Ordering};

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

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

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn dummy_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(Texture<'a>, Texture<'a>), String> {
    enum TextureColor {
        Yellow,
        White,
    };

    let mut square_texture1 = texture_creator
        .create_texture_target(None, 10, 10)
        .map_err(|e| e.to_string())?;
    let mut square_texture2 = texture_creator
        .create_texture_target(None, 10, 10)
        .map_err(|e| e.to_string())?;

    {
        let textures = vec![
            (&mut square_texture1, TextureColor::Yellow),
            (&mut square_texture2, TextureColor::White),
        ];

        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();
                match *user_context {
                    TextureColor::Yellow => {
                        println!("Yello");
                        for i in 0..10 {
                            for j in 0..10 {
                                if (i + j) % 4 == 0 {
                                    texture_canvas.set_draw_color(Color::RGB(255, 255, 0));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .expect("could not draw point");
                                }
                            }
                        }
                    }
                    TextureColor::White => {}
                };
            })
            .map_err(|e| e.to_string())?;
    }

    return Ok((square_texture1, square_texture2));
}


fn run_ea(input_count: u32, output_count: u32, pop_count: u64, iter_count: u64, fitness_func: &dyn Fn(&mut nn::Network)) -> () {
    println!("Pop count: {} {}", pop_count, iter_count);

    // initializeation of population. 

    let mut individual = nn::Network::new(input_count, output_count, true);

    fitness_func(&mut individual);

    for _ in 0..pop_count {
	
    }

    fitness_func(&mut individual);
	
}

fn main() -> std::result::Result<(), String> {
    let mut _asteroids_game = asteroids::game_init();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    canvas.clear();
    canvas.present();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let (square_texture1, _square_texture2) = dummy_texture(&mut canvas, &texture_creator)?;
    match canvas.copy(&square_texture1, None, Rect::new(0, 0, 10, 10)) {
        Ok(_) => {}
        Err(_) => {}
    }

    canvas.present();

    let population_count = 20;
    let mut _iteration_count = 0;
    let max_iter_count = 100000;
    let mut specific_pop: Vec<TestNetwork> = Vec::new();

    let input_node_count = 16;
    let output_node_count = 3;

    for _n in 1..population_count + 1 {
        let mut random_network = TestNetwork::new(input_node_count, output_node_count);
        random_network.update_fitness(&mut canvas);
        specific_pop.push(random_network);
    }

    // fitness evaluation
    let mut innovation_history = neat::InnovationHistory {
        global_inno_id: (input_node_count * output_node_count) as usize,
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
            if !found_spec {
                let mut new_spec = neat::Species::new(1.5, 0.8, 4.0);
                new_spec.set_champion(&test_n);
                species.push(new_spec);
            }
        }

        let mut offspring: Vec<TestNetwork> = Vec::new();

        let mut average_fit = 0.0;
        for pop in specific_pop.iter() {
            average_fit += pop.fitness();
        }
        average_fit /= specific_pop.len() as f64;
        println!("Average fitness: {}", average_fit);

        for spec in species.iter() {
            // add in the champ of the species in.
            offspring.push(TestNetwork::from_network(
                spec.champion.unwrap().network.clone(),
            ));
            let mut spec_av_fit = spec.average_fitness();
            println!("Spec av fit: {}", spec_av_fit);
            if spec_av_fit <= 0.0 {
                spec_av_fit = 1.0;
            }
            if average_fit <= 0.0 {
                average_fit = 1.0;
            }
            // -1 for champion
            println!("spec_av_fit / average_fit: {}", (spec_av_fit / average_fit));
            println!(
                "spec_av_fit / average_fit: {}",
                ((spec_av_fit / average_fit) * population_count as f64).floor()
            );
            let num_children =
                ((spec_av_fit / average_fit) * population_count as f64).floor() as u64 - 1;
            for _child_num in 0..num_children {
                let mut new_child =
                    TestNetwork::from_network(spec.generate_offspring(&innovation_history));
                new_child.custom_mutate(&mut innovation_history);
                offspring.push(new_child);
            }
        }

        let species_count = species.len();
        species.clear();

        for offpin in offspring.iter_mut() {
            offpin.update_fitness(&mut canvas);
        }

        specific_pop.append(&mut offspring);

        // // cull population
        specific_pop.sort_by_key(|indiv| Reverse((indiv.fitness() * 1000.0) as i128));
        specific_pop.truncate(population_count);

        assert!(specific_pop.len() == population_count);
        println!(
            "Species({}) average fitness {} number of innovations: {}",
            species_count,
            average_fit,
            innovation_history.conn_history.len()
        );
        average_history_per_iter.push(average_fit / (specific_pop.len() as f64));
    }

    // generate fitness values.

    specific_pop.sort_by_key(|indiv| Reverse((indiv.fitness() * 1000.0) as i128));
    let _top = &mut specific_pop[0].network;
    Ok(())
}

// todo look at this bench amrk thing https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust

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

        let test_network = TestNetwork::from_network(network.clone());
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

        let tmp_test = TestNetwork::from_network(network.clone());
        spec.set_champion(&tmp_test);
        assert!(spec.same_species(&network.edges));
        assert!(!spec.same_species(&network_two.edges));
        assert!(!spec.same_species(&network_three.edges));
    }
}
