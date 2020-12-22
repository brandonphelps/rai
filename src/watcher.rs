use std::{env, fs};


use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::event::Event;
use sdl2::EventPump;

use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use std::{thread, time};

use std::time::{Duration, Instant};
use std::collections::HashMap;

use rasteroids::asteroids;

mod neat;
mod nn;
mod distro;

fn draw_network(network: &nn::Network, canvas: &mut Canvas<Window>, x_offset: u32, y_offset: u64) {
    let mut nodes: Vec<&nn::Node> = Vec::new();
    let mut node_nums: Vec<u64> = Vec::new();
    let mut poses: Vec<(u32, u32)> = Vec::new();

    let tile_width = 20;
    let tile_height = 20; 
    let node_width = 10;
    let node_height = 10;

    canvas.set_draw_color(Color::RGB(255, 255, 0));

    // get max number of nodes in across all layers
    let mut max_num_nodes_in_layer = 0;
    for n in 0..network.layer_count {
	let nodes_per_layer = nn::node_per_layer(&network, n as u64).unwrap();
	if max_num_nodes_in_layer < nodes_per_layer  {
	    max_num_nodes_in_layer = nodes_per_layer;
	}
    }

    // draw grid
    for row_x in 0..network.layer_count {
	for col_y in 0..max_num_nodes_in_layer {
	    let pos_x = ((row_x * tile_width) + 5 as u32 * row_x + x_offset) as i32;
	    let pos_y = (((col_y * tile_height) + 5 * col_y) + y_offset) as i32;
	    canvas.fill_rect(Rect::new(pos_x,
				       pos_y,
				       tile_width as u32,
				       tile_height as u32));
	}
    }


    canvas.set_draw_color(Color::RGB(255, 0, 0));

    let mut node_index: u64 = 0;
    let mut node_pos = HashMap::new();
    
    
    // draw nodes in network
    for row_x in 0..network.layer_count {
	let nodes_per_layer = nn::node_per_layer(&network, row_x as u64).unwrap();
	for col_y in 0..nodes_per_layer {

	    let pos_x = (((row_x * tile_width) + 5 as u32 * row_x) + x_offset) as i32;
	    let pos_y = (((col_y * tile_height) + 5 * col_y) + y_offset) as i32;
	    canvas.fill_rect(Rect::new(pos_x,
				       pos_y,
				       node_width as u32,
				       node_height as u32));
	    node_pos.insert(node_index , (pos_x, pos_y));
	    node_index += 1;
	}
    }

    canvas.set_draw_color(Color::RGB(0, 0, 255));

    // draw the lines
    for edge in network.edges.iter() {
	let start_node = node_pos.get(&edge.from_node).unwrap();
	let end_node = node_pos.get(&edge.to_node).unwrap();
	
	canvas.draw_line(Point::new(start_node.0, start_node.1),
			 Point::new(end_node.0, end_node.1));
    }
}

fn read_file(filepath: &str) -> String {
    let contents = fs::read_to_string(filepath).expect("Somethign went wrong with reading form");
    return contents;
}

struct Generation {
    individuals: Vec<nn::Network>,
}

// loads a single playthrough of data from a directory.
// the directory must be in the form of "generation_num/<list of individuals>"
// each individual must be on nn::Network type. 
fn load_playthrough(directory: &str) -> HashMap<u64, Vec<nn::Network>> {
    let mut results = HashMap::new();

    for entry in fs::read_dir(directory).unwrap() {
	let entry = entry.unwrap();
	let path = entry.path();
	if path.is_dir() {
	    let gen_id = path.file_name().unwrap().to_str().unwrap().parse::<u64>().unwrap();
	    let mut individuals = Vec::new();
	    for individual in fs::read_dir(path).unwrap() {
		let ind_e = individual.unwrap();
		let ind_path = ind_e.path();
		if !ind_path.is_dir() { 
		    let network_str = read_file(ind_path.as_path().to_str().unwrap());
		    let network: nn::Network = serde_json::from_str(network_str.as_str()).unwrap();
		    individuals.push(network);
		}
	    }

	    
	    results.insert(gen_id, individuals);
	}
    }

    return results;
}


fn playthrough_asteroids(network: &mut nn::Network, canvas: &mut Canvas<Window>, event_pump: &mut EventPump) {
    let mut asteroids_game = asteroids::game_init();
    let mut duration = 0;

    let ten_millis = time::Duration::from_millis(10);
    let mut frame: u32 = 0;
    'running: loop {
        let start = Instant::now();
	for event in event_pump.poll_iter() {
	    match event {
		Event::Quit { .. }
		| Event::KeyDown {

		    keycode: Some(Keycode::Escape),
		    ..
		} => break 'running,
		_ => {},
	    }
	}

	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();

	draw_network(network, canvas, 400, 0);
	
	let game_input = distro::asteroids_thinker(network, &asteroids_game);

	asteroids_game = asteroids::game_update(&asteroids_game, (duration as f64) * 0.01, &game_input, canvas);

	if asteroids_game.game_over {
	    break;
	}

	canvas.present();
        duration = start.elapsed().as_millis();
    }

    
    // continue displaying the screent, the moment the game is over. 
    for i in 0..10000 {
	for event in event_pump.poll_iter() {
	    match event {
		Event::Quit { .. }
		| Event::KeyDown {

		    keycode: Some(Keycode::Escape),
		    ..
		} => break,
		_ => {},
	    }
	}
    }
}


pub fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];
    
    println!("Loading up data from: {}", filename);

    let playthrough = load_playthrough(filename);

    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Window", 800, 600)
	.position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
	.target_texture()
	.present_vsync()
        .build()
        .unwrap();

    canvas.clear();

    for gen in 3..50 {
	let network_list = playthrough.get(&gen).unwrap();

	for (index, network) in network_list.iter().enumerate() { 
	    if network.fitness() > 200.0 { 

		println!("Layer count: {}", network.layer_count);
		for i in 0..network.layer_count {
		    println!("Layer {} node count {}", i, nn::node_per_layer(&network, i as u64).unwrap());
		}

		println!("Playing generation: {}", gen);
		println!("playing network: {} which has fitness of {}", index, network.fitness());
		// let mut network = nn::Network::new(16, 3, true);
		// network.add_node(2, 1.0, 2.0, None);
		let mut copy_network = network.clone();
		playthrough_asteroids(&mut copy_network, &mut canvas, &mut event_pump);
	    }
	}
    }
}
