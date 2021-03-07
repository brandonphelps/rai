#![allow(dead_code)]

use std::time::{Instant};
use serde::{Deserialize, Serialize};


use rasteroids::asteroids;
use rasteroids::collision;

use crate::individual::Individual;
use crate::neat::InnovationHistory;
use crate::nn::Network;

// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! { 
//     static ref fitness_counter: Mutex<u32> = Mutex::new(0);
// }

use std::sync::atomic::{AtomicUsize, Ordering};

// temporary for debugging. 
static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);


// given a network, and a game state generate the next updates inputs.
pub fn asteroids_thinker(
    player: &Network,
    game_state: &asteroids::GameState,
) -> asteroids::GameInput {
    let mut vision_input: [f64; 8] = [100000.0; 8];

    // each item of vision is both a direction and distance to an asteroid.
    // the distance is from the ship, the network will have to figure out that
    // the order of the input is clockwise from north.
    for asteroid_dist in 1..30 {
        for ast in game_state.asteroids.iter() {
            let mut vision_c = collision::Circle {
                pos_x: 0.0,
                pos_y: 0.0,
                radius: 1.0,
            };

            if vision_input[0] == 100000.0 {
                vision_c.pos_x = game_state.player.rust_sux.pos_x + (asteroid_dist as f64);
                vision_c.pos_y = game_state.player.rust_sux.pos_y;
                if collision::collides(&vision_c, &ast.bounding_box()) {
                    vision_input[0] = asteroid_dist as f64;
                }
            }

            if vision_input[1] == 100000.0 {
                vision_c.pos_x = game_state.player.rust_sux.pos_x - (asteroid_dist as f64);
                vision_c.pos_y = game_state.player.rust_sux.pos_y;

                if collision::collides(&vision_c, &ast.bounding_box()) {
                    vision_input[1] = asteroid_dist as f64;
                }
            }
            if vision_input[2] == 100000.0 {
                vision_c.pos_x = game_state.player.rust_sux.pos_x;
                vision_c.pos_y = game_state.player.rust_sux.pos_y + (asteroid_dist as f64);
                if collision::collides(&vision_c, &ast.bounding_box()) {
                    vision_input[2] = asteroid_dist as f64;
                }
            }
            if vision_input[3] == 100000.0 {
                vision_c.pos_x = game_state.player.rust_sux.pos_x;
                vision_c.pos_y = game_state.player.rust_sux.pos_y - (asteroid_dist as f64);
                if collision::collides(&vision_c, &ast.bounding_box()) {
                    vision_input[3] = asteroid_dist as f64;
                }
            }
            if vision_input[4] == 100000.0 {
                vision_c.pos_x = game_state.player.rust_sux.pos_x + (asteroid_dist as f64);
                vision_c.pos_y = game_state.player.rust_sux.pos_y + (asteroid_dist as f64);
                if collision::collides(&vision_c, &ast.bounding_box()) {
                    vision_input[4] = asteroid_dist as f64;
                }
            }
            if vision_input[5] == 100000.0 {
                vision_c.pos_x = game_state.player.rust_sux.pos_x - (asteroid_dist as f64);
                vision_c.pos_y = game_state.player.rust_sux.pos_y - (asteroid_dist as f64);

                if collision::collides(&vision_c, &ast.bounding_box()) {
                    vision_input[5] = asteroid_dist as f64;
                }
            }
            if vision_input[6] == 100000.0 {
                vision_c.pos_x = game_state.player.rust_sux.pos_x + (asteroid_dist as f64);
                vision_c.pos_y = game_state.player.rust_sux.pos_y - (asteroid_dist as f64);

                if collision::collides(&vision_c, &ast.bounding_box()) {
                    vision_input[6] = asteroid_dist as f64;
                }
            }
            if vision_input[7] == 100000.0 {
                vision_c.pos_x = game_state.player.rust_sux.pos_x - (asteroid_dist as f64);
                vision_c.pos_y = game_state.player.rust_sux.pos_y + (asteroid_dist as f64);
                if collision::collides(&vision_c, &ast.bounding_box()) {
                    vision_input[7] = asteroid_dist as f64;
                }
            }
        }
    }

    let output = player.feed_input(vec![
        vision_input[0],
        vision_input[1],
        vision_input[2],
        vision_input[3],
        vision_input[4],
        vision_input[5],
        vision_input[6],
        vision_input[7],
    ]);
    assert_eq!(output.len(), 3);

    let mut game_input = asteroids::GameInput {
        shoot: false,
        thrusters: false,
        rotation: 0.0,
    };

    // do thinking
    if output[2] <= 0.5 {
        game_input.thrusters = true;
    }

    if output[1] <= 0.5 {
        game_input.shoot = true;
    }

    // todo: change this so that the ship doesn't need to turn.
    if output[0] <= 0.5 {
        game_input.rotation -= 0.39268;
    } else {
        game_input.rotation += 0.39268;
    }

    return game_input;
}

#[cfg(not(feature = "gui"))]
pub fn asteroids_fitness(player: &Network) -> f64 {
    
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);

    let mut asteroids_game = asteroids::game_init();
    let mut fitness: f64 = 0.0;

    let mut duration = 0;
    let max_turns = 100_000;
    for i in 0..max_turns {
        // vision
        let game_input = asteroids_thinker(player, &asteroids_game);

        // process action based on thinking
        asteroids_game =
            asteroids::game_update(&asteroids_game, (duration as f64) * 0.01, &game_input);
        let start = Instant::now();

        if asteroids_game.game_over {
            if asteroids_game.game_over_is_win {
                fitness = asteroids_game.score as f64;
            } else {
                fitness = asteroids_game.score as f64;
                fitness -= i as f64 * 0.01;
            }  
            break;
        }
        //thread::sleep(Duration::from_millis(10));
        duration = 1; // start.elapsed().as_millis();
    }
    if fitness <= 0.0 {
        fitness = 0.001;
    }
    return fitness;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AsteroidsPlayer {
    // thing of interest.
    pub brain: Network,
}

impl AsteroidsPlayer {
    pub fn new() -> Self {
        // note 8, 3 (input, output) must align with innovation history below.
        Self {
            brain: Network::new(8, 3, true),
        }
    }
}

impl Individual for AsteroidsPlayer {
    fn fitness(&self) -> f64 {
        asteroids_fitness(&self.brain)
    }

    fn ea_name(&self) -> String {
        String::from("rasteroids")
    }

    fn mutate<S>(&self, inno: &mut S) -> Self {
        let mut new_player = self.clone();
    //new_player.brain.mutate(inno);
        return new_player;
    }

    fn crossover<S>(&self, other: &Self,
		    _inno: &mut S) -> Self {
        println!("cross over");
        Self::new()
    }
}

impl Default for AsteroidsPlayer {
    fn default() -> Self {
        Self {
            brain: Network::new(8, 3, true),
        }
    }
}

pub struct AsteroidsStorage {
    inno_history: InnovationHistory,
}

impl AsteroidsStorage {
    pub fn new() -> Self {
        Self {
            inno_history: InnovationHistory::new(8, 3),
        }
    }
}
