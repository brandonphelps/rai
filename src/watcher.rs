
use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

mod nn;
mod neat;

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
	    if node.layer == layer as u64 {
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


pub fn main() {
    // let sdl_context = sdl2::init().unwrap();
    // let ttf_context = sdl2::ttf::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();
    // let window = video_subsystem
    //     .window("Window", 800, 600)
    //     .opengl()
    //     .build()
    //     .unwrap();

    // fn find_sdl_gl_driver() -> Option<u32> {
    //     for (index, item) in sdl2::render::drivers().enumerate() {
    //         if item.name == "opengl" {
    //             return Some(index as u32);
    //         }
    //     }
    //     None
    // }

    // let mut canvas = window
    //     .into_canvas()
    //     .index(find_sdl_gl_driver().unwrap())
    //     .build()
    //     .unwrap();

    // canvas.clear();

    // let font = ttf_context.load_font("lazy.ttf", 128).unwrap();

    // let texture_creator = canvas.texture_creator();

    // let target = Rect::new(100, 150, 300, 200);
    
}
