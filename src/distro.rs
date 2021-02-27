// common utils amonst job doers and job creators

use rasteroids::asteroids;
use rasteroids::collision;
use serde::{Deserialize, Serialize};

use crate::nn::Network;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobInfo {
    pub name: String,
    pub individual: Network,
    pub job_id: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResults {
    pub job_id: u128,
    pub fitness: f64,
}

#[allow(dead_code)]
pub struct EaFuncMap {}

#[cfg(not(feature = "gui"))]
impl EaFuncMap {
    #[allow(dead_code)]
    pub fn do_func(func_name: &String, indi: &Network) -> () {
        if func_name.as_str() == "rasteroids" {
            asteroids_fitness(indi);
        }
    }
}

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
        thread::sleep(Duration::from_millis(10));
        duration = start.elapsed().as_millis();
    }
    if fitness <= 0.0 {
        fitness = 0.001;
    }
    return fitness;
}
