#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]
use prgrs::{Length, Prgrs};
use rand::distributions::{Distribution, Normal};
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::cmp::Reverse;
use std::{thread, time};
use std::env;

use rasteroids::asteroids;
use rasteroids::collision;

mod evo_algo;
mod hrm;
mod neat;
mod nn;

use evo_algo::{Crossover, Individual};
use std::time::{Duration, Instant};

use std::sync::atomic::{AtomicUsize, Ordering};

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

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

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn speciate(population: &Vec<nn::Network>) -> Vec<neat::Species> {
    let mut species: Vec<neat::Species> = Vec::new();

    for test_n in population.iter() {
        let mut found_spec = false;

        for spec in species.iter_mut() {
            if spec.same_species(&test_n.edges) {
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

    return species;
}

fn draw_network(network: &nn::Network, canvas: &mut Canvas<Window>) {
    let mut nodes: Vec<&nn::Node> = Vec::new();
    let mut node_nums: Vec<u64> = Vec::new();
    let mut poses: Vec<(u32, u32)> = Vec::new();

    let width = 80; 
    let height = 20;
    let mut node_num = 0;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    
    for layer in 0..network.layer_count  { 
	
	let x: f64 = ((layer + 1) * width) as f64 / (network.layer_count+1) as f64;
	for (n_index, node) in network.nodes.iter().enumerate() {
	    if node.layer == layer.into() {
		let y: f64 = ((n_index + 1) * height) as f64 / (network.nodes.len() + 1) as f64;
		nodes.push(&node);
		node_nums.push(node_num);
		node_num += 1;
		let t = (x as u32, y as u32);
		poses.push(t);
	    }
	}
    }
    
    let offset = 100;

    for edge in network.edges.iter() {
	if edge.enabled {
	    
	}
    }


    for pose in poses.iter() {
	let _p = canvas.fill_rect(Rect::new(
	    (offset + pose.0 as i32) * 2, (offset + pose.1 as i32) * 2,
	    5, 5));
    }
}

fn asteroids_fitness(player: &mut nn::Network) -> () {
    let mut _fitness = 0.0;
    // self.network.pretty_print();

    let mut game_input = asteroids::GameInput {
        shoot: false,
        thrusters: false,
        rotation: 0.0,
    };

    let mut asteroids_game = asteroids::game_init();

    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
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

    let font = ttf_context.load_font("lazy.ttf", 128).unwrap();

    let texture_creator = canvas.texture_creator();

    let target = Rect::new(100, 150, 300, 200);


    // each item of vision is both a direction and distance to an asteroid.
    // the distance is from the ship, the network will have to figure out that
    // the order of the input is clockwise from north.
    let mut duration = 0;
    let max_turns = 3000;
    for _i in 0..max_turns {
	// vision
	
	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();

	let mut vision_input: [f64; 8] = [100000.0; 8];

	canvas.set_draw_color(Color::RGB(0, 255, 0));
	for asteroid_dist in 1..30 {
	    for ast in asteroids_game.asteroids.iter() {
		let mut vision_c = collision::Circle { pos_x: 0.0, pos_y: 0.0, radius: 1.0 };
		if vision_input[0] == 100000.0 { 
		    vision_c.pos_x = asteroids_game.player.rust_sux.pos_x + (asteroid_dist as f64);
		    vision_c.pos_y = asteroids_game.player.rust_sux.pos_y;
		    canvas.fill_rect(Rect::new(vision_c.pos_x as i32,
					       vision_c.pos_y as i32,
					       vision_c.radius as u32,
					       vision_c.radius as u32));
		    if collision::collides(&vision_c, &ast.bounding_box()) {
			vision_input[0] = asteroid_dist as f64;
		    }
		}
		if vision_input[1] == 100000.0 {
		    vision_c.pos_x = asteroids_game.player.rust_sux.pos_x - (asteroid_dist as f64);
		    vision_c.pos_y = asteroids_game.player.rust_sux.pos_y;
		    canvas.fill_rect(Rect::new(vision_c.pos_x as i32,
					       vision_c.pos_y as i32,
					       vision_c.radius as u32,
					       vision_c.radius as u32));
		    
		    if collision::collides(&vision_c, &ast.bounding_box()) {
			vision_input[1] = asteroid_dist as f64;
		    }
		}
		if vision_input[2] == 100000.0 {
		    vision_c.pos_x = asteroids_game.player.rust_sux.pos_x;
		    vision_c.pos_y = asteroids_game.player.rust_sux.pos_y + (asteroid_dist as f64);
		    canvas.fill_rect(Rect::new(vision_c.pos_x as i32,
					       vision_c.pos_y as i32,
					       vision_c.radius as u32,
					       vision_c.radius as u32));
		    
		    if collision::collides(&vision_c, &ast.bounding_box()) {
			vision_input[2] = asteroid_dist as f64;
		    }
		}
		if vision_input[3] == 100000.0 {
		    vision_c.pos_x = asteroids_game.player.rust_sux.pos_x;
		    vision_c.pos_y = asteroids_game.player.rust_sux.pos_y - (asteroid_dist as f64);
		    canvas.fill_rect(Rect::new(vision_c.pos_x as i32,
					       vision_c.pos_y as i32,
					       vision_c.radius as u32,
					       vision_c.radius as u32));
		    
		    if collision::collides(&vision_c, &ast.bounding_box()) {
			vision_input[3] = asteroid_dist as f64;
		    }
		}
		if vision_input[4] == 100000.0 { 
		    vision_c.pos_x = asteroids_game.player.rust_sux.pos_x + (asteroid_dist as f64);
		    vision_c.pos_y = asteroids_game.player.rust_sux.pos_y + (asteroid_dist as f64);
		    canvas.fill_rect(Rect::new(vision_c.pos_x as i32,
					       vision_c.pos_y as i32,
					       vision_c.radius as u32,
					       vision_c.radius as u32));
		    if collision::collides(&vision_c, &ast.bounding_box()) {
			vision_input[4] = asteroid_dist as f64;
		    }
		}
		if vision_input[5] == 100000.0 {
		    vision_c.pos_x = asteroids_game.player.rust_sux.pos_x - (asteroid_dist as f64);
		    vision_c.pos_y = asteroids_game.player.rust_sux.pos_y - (asteroid_dist as f64);
		    canvas.fill_rect(Rect::new(vision_c.pos_x as i32,
					       vision_c.pos_y as i32,
					       vision_c.radius as u32,
					       vision_c.radius as u32));
		    
		    if collision::collides(&vision_c, &ast.bounding_box()) {
			vision_input[5] = asteroid_dist as f64;
		    }
		}
		if vision_input[6] == 100000.0 {
		    vision_c.pos_x = asteroids_game.player.rust_sux.pos_x + (asteroid_dist as f64);
		    vision_c.pos_y = asteroids_game.player.rust_sux.pos_y - (asteroid_dist as f64);
		    canvas.fill_rect(Rect::new(vision_c.pos_x as i32,
					       vision_c.pos_y as i32,
					       vision_c.radius as u32,
					       vision_c.radius as u32));
		    
		    if collision::collides(&vision_c, &ast.bounding_box()) {
			vision_input[6] = asteroid_dist as f64;
		    }
		}
		if vision_input[7] == 100000.0 {
		    vision_c.pos_x = asteroids_game.player.rust_sux.pos_x - (asteroid_dist as f64);
		    vision_c.pos_y = asteroids_game.player.rust_sux.pos_y + (asteroid_dist as f64);
		    canvas.fill_rect(Rect::new(vision_c.pos_x as i32,
					       vision_c.pos_y as i32,
					       vision_c.radius as u32,
					       vision_c.radius as u32));
		    
		    if collision::collides(&vision_c, &ast.bounding_box()) {
			vision_input[7] = asteroid_dist as f64;
		    }
		}
	    }
	}
	
	let output = player.feed_input(vec![vision_input[0],
					    vision_input[1],
					    vision_input[2],
					    vision_input[3],
					    vision_input[4],
					    vision_input[5],
					    vision_input[6],
					    vision_input[7]]);
	assert_eq!(output.len(), 3);

	// do thinking 
        if output[2] <= 0.5 {
            game_input.thrusters = true;
        }

        if output[1] <= 0.5 {
            game_input.shoot = true;
        }

        game_input.rotation = output[0];
	
	let surface = font.render(&output[1].to_string()).blended(Color::RGBA(255, 0, 0, 255)).unwrap();
	let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

	draw_network(&player, &mut canvas);

	canvas.copy(&texture, None, Some(target)).unwrap();

	// process action based on thinking
        asteroids_game = asteroids::game_update(
            &asteroids_game,
            (duration as f64) * 0.01,
            &game_input
        );
        let start = Instant::now();
        canvas.present();

        if asteroids_game.game_over {
            if asteroids_game.game_over_is_win {
                player.fitness = 1000000.0;
            } else {
                player.fitness = (_i as f64 / max_turns as f64) as f64;
            }
            break;
        }

        thread::sleep(Duration::from_millis(10));
        duration = start.elapsed().as_millis();
        game_input.shoot = false;
        game_input.thrusters = false;
    }
}


fn run_ea(input_count: u32, output_count: u32, pop_count: u64, iter_count: u64, fitness_func: &dyn Fn(&mut nn::Network)) -> () {
    println!("Pop count: {} {}", pop_count, iter_count);

    let mut average_history_per_iter: Vec<f64> = Vec::new();

    // initializeation.
    // population holder. 
    let mut specific_pop: Vec<nn::Network> = Vec::new();
    let mut individual = nn::Network::new(input_count, output_count, true);
    // fitness evaluation
    let mut innovation_history = neat::InnovationHistory {
        global_inno_id: (input_count * output_count) as usize,
        conn_history: vec![],
    };

    fitness_func(&mut individual);

    for _ in 0..pop_count+1 {
	specific_pop.push(individual.clone());
    }


    for generation in 0..iter_count {
	println!("Generation: {}", generation);

	// move to speciate function
	// specization. divide the population into different species. 
        // why can't this forloop be outside this forloop? something
        // about the specific_pop updating is mutable borrow after an immutable barrow on something?

	let mut species = speciate(&specific_pop); 
	let species_count = species.len();
	println!("Species count: {}", species_count);

	let mut offspring = Vec::new();

	// there is prob some vector function for this or something with a closure? 
	let mut average_fit = 0.0;
	for ind in specific_pop.iter() {
	    average_fit += ind.fitness();
	}
	average_fit /= specific_pop.len() as f64;

	println!("Average fitness: {}", average_fit);

	// generate offsprint from each of the species.
	// the number of offspring depends on the average fitness of the species. 
        for spec in species.iter() {
            // add in the champ of the species in.
            offspring.push(spec.champion.unwrap().clone());
            let mut spec_av_fit = spec.average_fitness();
            println!("Spec av fit: {}", spec_av_fit);
            if spec_av_fit <= 0.0 {
                spec_av_fit = 1.0;
            }
            if average_fit <= 0.0 {
                average_fit = 1.0;
            }

            let num_children = num_child_to_make(average_fit, spec_av_fit, pop_count);

            for _child_num in 0..num_children {
                let mut new_child = spec.generate_offspring(&innovation_history).clone();
                new_child.mutate(&mut innovation_history);
		fitness_func(&mut new_child);
		println!("Evaluting child: {} {}", _child_num, new_child.fitness());
                offspring.push(new_child);
            }
        }


	species.clear();


	for _ind in offspring.iter_mut() {

	}

	specific_pop.append(&mut offspring);

        // // cull population
        specific_pop.sort_by_key(|indiv| Reverse((indiv.fitness() * 1000.0) as i128));
        specific_pop.truncate(pop_count as usize);

        assert!(specific_pop.len() == pop_count as usize);
        println!(
            "Species({}) average fitness {} number of innovations: {}",
            species_count,
            average_fit,
            innovation_history.conn_history.len()
        );
        average_history_per_iter.push(average_fit / (specific_pop.len() as f64));
    }

    specific_pop.sort_by_key(|indiv| Reverse((indiv.fitness() * 1000.0) as i128));
    let _top = &mut specific_pop[0];
}


fn main() -> std::result::Result<(), String> {
    let population_count = 200;
    let max_iter_count = 10000;
    let input_node_count = 16;
    let output_node_count = 3;

    let _args: Vec<_> = env::args().collect();

    run_ea(input_node_count, output_node_count,
	   population_count, max_iter_count, &asteroids_fitness);

    Ok(())
}

// determine the number of children to create based on the total fitness and the specifies fitness. 
fn num_child_to_make(total_fitness: f64, species_fitness: f64, total_population: u64) -> u64 {
    println!("Total: ({}) Spec: ({}) Pop: ({})", total_fitness, species_fitness, total_population);
    assert!(total_fitness >= species_fitness);
    ((species_fitness / total_fitness) * total_population as f64) as u64
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

        let test_network = network.clone();
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

        let tmp_test = network.clone();
        spec.set_champion(&tmp_test);
        assert!(spec.same_species(&network.edges));
        assert!(!spec.same_species(&network_two.edges));
        assert!(!spec.same_species(&network_three.edges));
    }

    #[test]
    fn test_child_num_people() {
	let total_fitness = 100.0;
	let total_pop = 100;

	assert_eq!(num_child_to_make(total_fitness, 100.0, total_pop), 100);

	// if a species has half the total pop then it should contribute to half the population.
	assert_eq!(num_child_to_make(total_fitness, 50.0, total_pop), 50);

	assert_eq!(num_child_to_make(total_fitness, 2.0, total_pop), 2);
	assert_eq!(num_child_to_make(total_fitness, 50.0, 200), 100);
    }
}
