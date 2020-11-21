#![allow(unused_variables)]
#![allow(unused_mut)]


#[derive(Debug, Clone)]
struct MoveAblePos {
    pos_x: f64,
    pos_y: f64,
    velocity: f64,
    /// can only be values of 0 -> 2PI.
    direction: f64,
}

#[derive(Clone, Debug)]
struct Asteroid {
    rust_sux: MoveAblePos,
}

#[derive(Clone, Debug)]
struct Player {
    rust_sux: MoveAblePos,
}


#[derive(Clone, Debug)]
struct Bullet {
    rust_sux: MoveAblePos,
    /// amount of update time the bullet will exists for. 
    life_time: f64,
}

#[derive(Clone, Debug)]
struct GameState {
    asteroids: Vec<Asteroid>,
    player: Player,
    bullets: Vec<Bullet>,
}

struct GameInput {
    // radians value for what to update the ship with. 
    rotation: f64,

    // if true then the player is wanting to shoot a bullet
    shoot: bool,

    // if true then player is wanting to move forward. 
    thrusters: bool,
}

fn game_init() -> GameState {
    return GameState {
        asteroids: vec![],
        player: Player {
            rust_sux:  MoveAblePos {
                pos_x: 0.0,
                pos_y: 0.0,
                velocity: 0.0,
                direction: 0.0,
            }
        },
        bullets: vec![],
    };
}


// update to have wrap around the world.
fn update_pos(r: &mut MoveAblePos, dt: f64) -> () {
    r.pos_x += dt * r.velocity * (r.direction).cos();
    r.pos_y += dt * r.velocity * (r.direction).sin();
}
    
// called when the player wishes to shoot a bullet
fn shoot_bullet(game_state: &mut GameState ) -> () {
    let p = &game_state.player;
    let bullet = Bullet {
        // maybe could clone the players MoveAblePos
        rust_sux : MoveAblePos {
            pos_x: p.rust_sux.pos_x,
            pos_y: p.rust_sux.pos_y,
            velocity: p.rust_sux.velocity + 2.0,
            direction: p.rust_sux.direction
        },
        life_time: 30.0,
    };

    game_state.bullets.push(bullet);
}

fn game_update(game_state: &GameState, dt: f64, game_input: &GameInput) -> GameState {
    let mut new_state = game_state.clone();

    if game_input.shoot {
        shoot_bullet(&mut new_state);
    }

    new_state.player.rust_sux.direction += game_input.rotation * dt;
    if new_state.player.rust_sux.direction > std::f64::consts::PI {
        new_state.player.rust_sux.direction -= std::f64::consts::PI;
    }

    if new_state.player.rust_sux.direction < 0.0 {
        new_state.player.rust_sux.direction += std::f64::consts::PI;
    }

    let mut player = &mut new_state.player;

    update_pos(&mut player.rust_sux, dt);

    for ast in new_state.asteroids.iter_mut() {
        update_pos(&mut ast.rust_sux, dt);
    }

    for bull in new_state.bullets.iter_mut() {
        update_pos(&mut bull.rust_sux, dt);
    }

    return new_state;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Distribution, Normal};
    use rand::prelude::*;
    use rand::seq::SliceRandom;

    #[test]
    fn test_initialization() {
        let mut game_state = game_init();
        let mut game_input = GameInput{
            shoot: false,
            thrusters: false,
            rotation: 0.0,
        };

        let mut rng = rand::thread_rng();


        game_state.player.rust_sux.velocity = 0.1;
        game_state.player.rust_sux.direction = std::f64::consts::PI * 0.5;


        for i in 0..10 {
            if rng.gen::<f64>() < 0.2 {
                // shoot_bullet(&mut game_state);
                game_input.shoot = true;
            }
            
            game_input.rotation = 0.4;

            game_state = game_update(&game_state, 1.0, &game_input);
            println!("{:#?}", game_state);
            game_input.shoot = false;
        }
    }

    #[test]
    fn test_pos_zero_vec() {
        let mut pos_thing = MoveAblePos {
            pos_x: 0.0,
            pos_y: 0.0,
            velocity: 0.0,
            direction: 0.0,
        };

        update_pos(&mut pos_thing, 1.0);
        assert_eq!(pos_thing.pos_x, 0.0);
        assert_eq!(pos_thing.pos_y, 0.0);

    }

    #[test]
    fn test_pos_vec_one_zero_dir() {
        let mut pos_thing = MoveAblePos {
            pos_x: 0.0,
            pos_y: 0.0,
            velocity: 1.0,
            direction: 0.0,
        };

        update_pos(&mut pos_thing, 1.0);
        assert_eq!(pos_thing.pos_x, 1.0);
        assert_eq!(pos_thing.pos_y, 0.0);

    }

    #[test]
    fn test_pos_vec_one_90_dir() {
        let mut pos_thing = MoveAblePos {
            pos_x: 0.0,
            pos_y: 0.0,
            velocity: 1.0,
            direction: std::f64::consts::PI * 0.5,
        };

        update_pos(&mut pos_thing, 1.0);
        assert!(pos_thing.pos_x < 0.00001);
        assert!(pos_thing.pos_x > -0.0001);
        assert!(pos_thing.pos_y == 1.0);
    }
}

