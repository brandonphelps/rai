#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::collision;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct MoveAblePos {
    pub pos_x: f64,
    pub pos_y: f64,
    velocity: f64,
    /// can only be values of 0 -> 2PI.
    direction: f64,
}

#[derive(Clone, Debug)]
pub struct Asteroid {
    rust_sux: MoveAblePos,
    radius: f64,
}

impl Asteroid {
    pub fn bounding_box(&self) -> collision::Circle {
        return collision::Circle {
            pos_x: self.rust_sux.pos_x,
            pos_y: self.rust_sux.pos_y,
            radius: self.radius,
        };
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub rust_sux: MoveAblePos,
    radius: f64,
}
impl Player {
    pub fn bounding_box(&self) -> collision::Circle {
        return collision::Circle {
            pos_x: self.rust_sux.pos_x,
            pos_y: self.rust_sux.pos_y,
            radius: 2.0,
        };
    }
}

#[derive(Clone, Debug)]
struct Bullet {
    rust_sux: MoveAblePos,
    /// amount of update time the bullet will exists for.
    life_time: f64,
    radius: f64,
}

impl Bullet {
    fn bounding_box(&self) -> collision::Circle {
        return collision::Circle {
            pos_x: self.rust_sux.pos_x,
            pos_y: self.rust_sux.pos_y,
            radius: self.radius,
        };
    }
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub asteroids: Vec<Asteroid>,
    pub player: Player,
    bullets: Vec<Bullet>,
    shoot_bullet_cd: i16,
    world_width: f64,
    world_height: f64,
    // if true then the game is finished.
    pub game_over: bool,
    pub game_over_is_win: bool,
    score: u64,
}

pub struct GameInput {
    // radians value for what to update the ship with.
    pub rotation: f64,

    // if true then the player is wanting to shoot a bullet
    pub shoot: bool,

    // if true then player is wanting to move forward.
    pub thrusters: bool,
}

pub fn game_init() -> GameState {
    let mut game_state = GameState {
        asteroids: vec![],
        game_over: false,
        game_over_is_win: false,
        player: Player {
            rust_sux: MoveAblePos {
                pos_x: 50.0,
                pos_y: 50.0,
                velocity: 0.0,
                direction: 0.0,
            },
            radius: 5.0,
        },
        bullets: vec![],
        world_width: 100.0,
        world_height: 100.0,
	shoot_bullet_cd: 0,
	score: 0,
    };

    let mut rng = rand::thread_rng();

    for _i in 0..rng.gen_range(5, 10) {
        game_state.asteroids.push(Asteroid {
            rust_sux: MoveAblePos {
                pos_x: rng.gen_range(10.0, 50.0),
                pos_y: rng.gen_range(10.0, 50.0),
                velocity: rng.gen_range(1.0, 2.0),
                direction: rng.gen_range(0.0, std::f64::consts::PI),
            },
            radius: 8.0,
        });
    }
    return game_state;
}

fn update_pos(r: &mut MoveAblePos, dt: f64, world_width: f64, world_height: f64) {
    r.pos_x += dt * r.velocity * (r.direction).cos();
    r.pos_y += dt * r.velocity * (r.direction).sin();

    if r.pos_x > world_width {
        r.pos_x = 0.0;
    }
    if r.pos_y > world_height {
        r.pos_y = 0.0;
    }
    if r.pos_x < 0.0 {
        r.pos_x = world_width;
    }
    if r.pos_y < 0.0 {
        r.pos_y = world_height;
    }
}

// called when the player wishes to shoot a bullet
fn shoot_bullet(game_state: &mut GameState) -> () {
    let p = &game_state.player;
    let bullet = Bullet {
        // maybe could clone the players MoveAblePos
        rust_sux: MoveAblePos {
            pos_x: p.rust_sux.pos_x,
            pos_y: p.rust_sux.pos_y,
            velocity: p.rust_sux.velocity + 2.0,
            direction: p.rust_sux.direction,
        },
        life_time: 10.0,
        radius: 2.0,
    };

    game_state.bullets.push(bullet);
}

pub fn game_update(
    game_state: &GameState,
    dt: f64,
    game_input: &GameInput,
    canvas: &mut Canvas<Window>,
) -> GameState {

    let mut new_state = game_state.clone();

    let pixels_to_meters = 10;

    new_state.shoot_bullet_cd = game_state.shoot_bullet_cd - 1;

    if new_state.shoot_bullet_cd < 0 {
	new_state.shoot_bullet_cd = 0;
    }

    if game_input.shoot && new_state.shoot_bullet_cd == 0 {
        shoot_bullet(&mut new_state);
	// todo: what should the cd be? 
	new_state.shoot_bullet_cd = 10;
    }

    if game_input.thrusters {
        new_state.player.rust_sux.velocity = 2.0;
    } else {
        // need some sort of decay
        new_state.player.rust_sux.velocity = 0.0;
    }

    // todo: add in wrap around for bullets and asteroids and player etc.
    new_state.player.rust_sux.direction += game_input.rotation * dt;

    if new_state.player.rust_sux.direction > 2.0 * std::f64::consts::PI {
        new_state.player.rust_sux.direction -= 2.0 * std::f64::consts::PI;
    }

    if new_state.player.rust_sux.direction < 0.0 {
        new_state.player.rust_sux.direction += 2.0 * std::f64::consts::PI;
    }

    let mut player = &mut new_state.player;

    update_pos(
        &mut player.rust_sux,
        dt,
        game_state.world_width,
        game_state.world_height,
    );

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    for ast in new_state.asteroids.iter_mut() {
        update_pos(
            &mut ast.rust_sux,
            dt,
            game_state.world_width,
            game_state.world_height,
        );
    }

    for bull in new_state.bullets.iter_mut() {
        update_pos(
            &mut bull.rust_sux,
            dt,
            game_state.world_width,
            game_state.world_height,
        );
        bull.life_time -= 1.0 * dt;
    }

    new_state.bullets.retain(|bull| bull.life_time > 0.0);

    // check for collision
    let mut new_asteroids = Vec::new();

    // update for asteroids and bullets.
    for ast in new_state.asteroids.iter() {
        let mut deleted_aster = false;
        // todo: switch to filter on lifetime and can move retain to after this double loop?
        for bull in new_state.bullets.iter_mut() {
            if collision::collides(&ast.bounding_box(), &bull.bounding_box()) {
                // break the asteroid into two, and give some random direction and velocity.
                // remove the bullet.

                // only make new asteroids from those that are large enough.
                // large asteroid
                if ast.radius > 3.0 {
                    // add two asteroids.
                    new_asteroids.push(Asteroid {
                        rust_sux: MoveAblePos {
                            pos_x: ast.rust_sux.pos_x,
                            pos_y: ast.rust_sux.pos_y,
                            // todo: change this at some point.
                            velocity: ast.rust_sux.velocity - 0.1,
                            direction: ast.rust_sux.direction,
                        },
                        radius: ast.radius / 2.0,
                    });

                    new_asteroids.push(Asteroid {
                        rust_sux: MoveAblePos {
                            pos_x: ast.rust_sux.pos_x,
                            pos_y: ast.rust_sux.pos_y,
                            // todo: change this at some point.
                            velocity: ast.rust_sux.velocity + 0.1,
                            // send this one in the opposite direction.
                            direction: (ast.rust_sux.direction + std::f64::consts::PI * 0.5),
                        },
                        radius: 3.0,
                    });
                }
                deleted_aster = true;
		// 100 points per asteroid killed. 
		new_state.score += 100;
                bull.life_time = 0.0;
                break;
            }
        }
        // is this good here? wouldn't want a bullet to
        // be able to kill two asteroids right?
        new_state.bullets.retain(|bull| bull.life_time > 0.0);

        if !deleted_aster {
            new_asteroids.push(ast.clone());
        }
    }

    // update for player asteroid collision.
    for ast in new_state.asteroids.iter() {
        if collision::collides(&ast.bounding_box(), &new_state.player.bounding_box()) {
            new_state.game_over = true;
            break;
        }
    }

    new_state.asteroids = new_asteroids;

    if new_state.asteroids.len() == 0 {
        new_state.game_over = true;
        new_state.game_over_is_win = true;
    }

    // put this into a asteroids specific draw function.

    for ast in new_state.asteroids.iter() {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let p = canvas.fill_rect(Rect::new(
            ast.rust_sux.pos_x as i32,
            ast.rust_sux.pos_y as i32,
            ast.radius as u32,
            ast.radius as u32,
        ));
        match p {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    for bull in new_state.bullets.iter() {
        canvas.set_draw_color(Color::RGB(125, 125, 0));
        let p = canvas.fill_rect(Rect::new(
            bull.rust_sux.pos_x as i32,
            bull.rust_sux.pos_y as i32,
            bull.radius as u32,
            bull.radius as u32,
        ));
        match p {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    let p = canvas.fill_rect(Rect::new(
        new_state.player.rust_sux.pos_x as i32,
        new_state.player.rust_sux.pos_y as i32,
        new_state.player.radius as u32,
        new_state.player.radius as u32,
    ));

    match p {
        Ok(_) => {}
        Err(_) => {}
    }
    return new_state;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_zero_vec() {
        let mut pos_thing = MoveAblePos {
            pos_x: 0.0,
            pos_y: 0.0,
            velocity: 0.0,
            direction: 0.0,
        };

        update_pos(&mut pos_thing, 1.0, 100.0, 100.0);
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

        update_pos(&mut pos_thing, 1.0, 100.0, 100.0);
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

        update_pos(&mut pos_thing, 1.0, 100.0, 100.0);
        assert!(pos_thing.pos_x < 0.00001);
        assert!(pos_thing.pos_x > -0.0001);
        assert!(pos_thing.pos_y == 1.0);
    }
}
