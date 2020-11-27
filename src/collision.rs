/// helper functions and objects for 2d collision detect.

struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug)]
pub struct Circle {
    pub pos_x: f64,
    pub pos_y: f64,
    pub radius: f64,
}

struct Rectangle {
    // upper left
    p_ul: Point,

    // upper right
    p_ur: Point,

    // lower left
    p_ll: Point,

    // lower right
    p_lr: Point,
}

// given three colinear points checks if point q lines on line segment pr
fn point_on_segement(p: &Point, q: &Point, r: &Point) -> bool {
    let max_x;
    let min_x;
    let max_y;
    let min_y;

    if p.x < r.x {
        max_x = r.x;
        min_x = p.x;
    } else {
        max_x = p.x;
        min_x = r.x;
    }

    if p.y < r.y {
        max_y = r.y;
        min_y = p.y;
    } else {
        max_y = p.y;
        min_y = r.y;
    }

    if q.x <= max_x && q.x >= min_x && q.y <= max_y && q.y >= min_y {
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
    } else if val < 0.0 {
        // counter clock wise orientation
        return 2;
    } else {
        return 0;
    }
}

fn line_intersect(p_a1: &Point, p_a2: &Point, p_b1: &Point, p_b2: &Point) -> bool {
    let o1 = point_orientation(p_a1, p_a2, p_b1);
    let o2 = point_orientation(p_a1, p_a2, p_b2);
    let o3 = point_orientation(p_b1, p_b2, p_a1);
    let o4 = point_orientation(p_b1, p_b2, p_a2);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    if o1 == 0 && point_on_segement(p_a1, p_b1, p_a2) {
        return true;
    }

    if o2 == 0 && point_on_segement(p_a1, p_b2, p_a2) {
        return true;
    }

    if o3 == 0 && point_on_segement(p_b1, p_a2, p_b2) {
        return true;
    }

    if o4 == 0 && point_on_segement(p_b1, p_a2, p_b2) {
        return true;
    }

    return false;
}

pub fn collides(circle_one: &Circle, circle_two: &Circle) -> bool {
    let dist_x = circle_one.pos_x - circle_two.pos_x;
    let dist_y = circle_one.pos_y - circle_two.pos_y;
    let dist = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
    return dist <= circle_one.radius + circle_two.radius;
}

fn collides_rectangles(_rect_one: &Rectangle, _rect_two: &Rectangle) -> bool {
    return false;
}
