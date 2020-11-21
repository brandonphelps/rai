#![allow(unused_variables)]
#![allow(unused_mut)]

struct Point {
    x: f64,
    y: f64,
}

struct Circle {
    pos_x: f64,
    pos_y: f64,
    radius: f64
}

struct Rectangle {
    // upper left
    pos_x_ul: f64,
    pos_y_ul: f64,

    // upper right
    pos_x_ur: f64,
    pos_y_ur: f64,

    // lower left 
    pos_x_ll: f64,
    pos_y_ll: f64,

    // lower right
    pos_x_lr: f64,
    pos_y_lr: f64,
}

// given three colinear points checks if point q lines on line segment pr
fn point_on_segement(p: &Point, q: &Point, r: &Point) -> bool {
    let mut max_x;
    let mut min_x;
    let mut max_y;
    let mut min_y;

    if p.x < r.x {
        max_x = r.x;
        min_x = p.x;
    }
    else {
        max_x = p.x;
        min_x = r.x;
    }

    if p.y < r.y {
        max_y = r.y;
        min_y = p.y;
    }
    else {
        max_y = p.y;
        min_y = r.y;
    }

    if q.x <= max_x && q.x >= min_x &&
        q.y <= max_y && q.y >= min_y {
        return true;
    }
    return false;
}

// todo update to be enum return
fn point_orientation(p: &Point, q: &Point, r: &Point) -> u8 {
    let val = ((q.y - p.y) * (r.x - q.x)) - ((q.x - p.x) * (r.y - q.y));
    if val > 0.0 {
        // clock wise orientation
        return 1;
    }
    else if val < 0.0 {
        // counter clock wise orientation
        return 2;
    }
    else {
        return 0;
    }
}

fn line_intersect(pA1: &Point, pA2: &Point, pB1: &Point, pB2: &Point) -> bool {
    let o1 = point_orientation(pA1, pA2, pB1);
    let o2 = point_orientation(pA1, pA2, pB2);
    let o3 = point_orientation(pB1, pB2, pA1);
    let o4 = point_orientation(pB1, pB2, pA2);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    if o1 == 0 && point_on_segement(pA1, pB1, pA2) {
        return true
    }

    if o2 == 0 && point_on_segement(pA1, pB2, pA2) {
        return true
    }

    return false
}
    

fn collides(circle_one: &Circle,
            circle_two: &Circle) -> bool {
    
    let dist_x = circle_one.pos_x - circle_two.pos_x;
    let dist_y = circle_one.pos_y - circle_two.pos_y;
    let dist = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
    return dist <= circle_one.radius + circle_two.radius;
}

// fn collides(rect_one: &Rectangle,
//             rect_two: &Rectangle) -> bool {
//     return false;
// }

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
    world_width: f64,
    world_height: f64,
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
        world_width: 100.0,
        world_height: 100.0,
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
        bull.life_time -= 1.0 * dt;
    }

    new_state.bullets.retain(|bull| bull.life_time > 0.0);

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


        for i in 0..50 {
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


    #[test]
    fn test_colliding_circles() {
        let circle_one = Circle {
            pos_x: 0.0,
            pos_y: 0.0,
            radius: 1.0 };
        
        let circle_two = Circle {
            pos_x: 0.0,
            pos_y: 0.0,
            radius: 1.0 };


        let circle_three = Circle {
            pos_x: 0.5,
            pos_y: 0.0,
            radius: 1.0 
        };


        let circle_four = Circle {
            pos_x: 10.0,
            pos_y: 10.0,
            radius: 2.0,
        };

        assert_eq!(collides(&circle_one, &circle_two), true);
        assert_eq!(collides(&circle_one, &circle_three), true);
        assert_eq!(collides(&circle_one, &circle_four), false);
        assert_eq!(collides(&circle_three, &circle_four), false);
    }

    #[test]
    fn test_colliding_lines() {
        let mut p1 = Point{ x:1.0, y:1.0 };
        let mut q1 = Point{ x:10.0, y:1.0 };
        let mut p2 = Point{ x:1.0, y:2.0 };
        let mut q2 = Point{ x:10.0, y:2.0 };

        assert_eq!(line_intersect(&p1, &q1, &p2, &q2), false);

        p1 = Point{ x:10.0, y:0.0 }; 
        q1 = Point{ x:0.0, y:10.0 };
        p2 = Point{ x:0.0, y:0.0 }; 
        q2 = Point{ x:10.0, y:10.0 };

        assert_eq!(line_intersect(&p1, &q1, &p2, &q2), true);
    }
}

