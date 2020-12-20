use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::event::Event;

use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use std::{thread, time};

use std::collections::HashMap;

mod neat;
mod nn;

fn draw_network(network: &nn::Network, canvas: &mut Canvas<Window>) {
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
	    canvas.fill_rect(Rect::new(((row_x * tile_width) + 5 as u32 * row_x) as i32,
				       ((col_y * tile_height) + 5 * col_y) as i32,
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

	    let pos_x = ((row_x * tile_width) + 5 as u32 * row_x) as i32;
	    let pos_y = ((col_y * tile_height) + 5 * col_y) as i32;
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
    
    // let width = 80;
    // let height = 20;
    // let mut node_num = 0;

    // canvas.set_draw_color(Color::RGB(255, 0, 0));

    // for layer in 0..network.layer_count {
    //     let x: f64 = ((layer + 1) * width) as f64 / (network.layer_count + 1) as f64;
    //     for (n_index, node) in network.nodes.iter().enumerate() {
    //         if node.layer == layer as u64 {
    //             let y: f64 = ((n_index + 1) * height) as f64 / (network.nodes.len() + 1) as f64;
    //             nodes.push(&node);
    //             node_nums.push(node_num);
    //             node_num += 1;
    //             let t = (x as u32, y as u32);
    //             poses.push(t);
    //         }
    //     }
    // }

    // let offset = 100;

    // for edge in network.edges.iter() {
    //     if edge.enabled {}
    // }

    // for pose in poses.iter() {
    //     let _p = canvas.fill_rect(Rect::new(
    //         (offset + pose.0 as i32) * 2,
    //         (offset + pose.1 as i32) * 2,
    //         5,
    //         5,
    //     ));
    // }
}

// let surface = font.render(&output[1].to_string()).blended(Color::RGBA(255, 0, 0, 255)).unwrap();
// let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

// draw_network(&player, &mut canvas);
// canvas.copy(&texture, None, Some(target)).unwrap();

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Window", 800, 600)
	.position_centered()
        .build()
        .unwrap();

    fn find_sdl_gl_driver() -> Option<u32> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Some(index as u32);
            }
        }
        None
    }

    let mut canvas = window
        .into_canvas()
	.target_texture()
	.present_vsync()
        .build()
        .unwrap();

    canvas.clear();

    let mut network = nn::Network::new(10, 4, true);

    network.add_node(2, 1.0, 2.0, None);
    
    // let font = ttf_context.load_font("lazy.ttf", 128).unwrap();

    // let texture_creator = canvas.texture_creator();

    // let target = Rect::new(100, 150, 300, 200);

    let ten_millis = time::Duration::from_millis(10);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame: u32 = 0;
    'running: loop {
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

	draw_network(&network, &mut canvas);


	canvas.present();

	thread::sleep(ten_millis);
    }

}
