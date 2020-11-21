#![allow(unused_variables)]
#![allow(unused_mut)]

#[derive(Debug, Clone)]
struct MoveAblePos {
    pos_x: f64,
    pos_y: f64,
    velocity: f64,
    /// can only be values of 0 -> 360. maybe should be in radians.
    direction: f64,
}

#[derive(Clone)]
struct Asteroid {
    rust_sux: MoveAblePos,
}

#[derive(Clone, Debug)]
struct Player {
    rust_sux: MoveAblePos,
}


#[derive(Clone)]
struct Bullet {
    rust_sux: MoveAblePos,
    /// amount of update time the bullet will exists for. 
    life_time: f64,
}

#[derive(Clone)]
struct GameState {
    asteroids: Vec<Asteroid>,
    player: Player,
    bullets: Vec<Bullet>,
}

struct GameInput {

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


fn update_pos(r: &mut MoveAblePos, dt: f64) -> () {

    r.pos_x += dt * r.velocity * (r.direction).cos();
    r.pos_y += dt * r.velocity * (r.direction).sin();
}
    

fn game_update(game_state: &GameState, dt: f64, game_input: &GameInput) -> GameState {
    let mut new_state = game_state.clone();

    let mut player = &mut new_state.player;
    update_pos(&mut player.rust_sux, dt);
    return new_state;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let mut game_state = game_init();
        let game_input = GameInput{ };

        game_state = game_update(&game_state, 1.0, &game_input);
        println!("{:#?}", game_state.player);
        game_state.player.rust_sux.direction = 90.0;
        game_state = game_update(&game_state, 1.0, &game_input);
        println!("{:#?}", game_state.player);
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

