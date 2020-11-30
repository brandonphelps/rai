/// helper functions and objects for 2d collision detect.

#[allow(dead_code)]
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

#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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

#[allow(dead_code)]
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colliding_circles() {
        let circle_one = Circle {
            pos_x: 0.0,
            pos_y: 0.0,
            radius: 1.0,
        };

        let circle_two = Circle {
            pos_x: 0.0,
            pos_y: 0.0,
            radius: 1.0,
        };

        let circle_three = Circle {
            pos_x: 0.5,
            pos_y: 0.0,
            radius: 1.0,
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
    fn test_colliding_rectangles() {
        let _r1 = Rectangle {
            p_ul: Point { x: 0.0, y: 0.0 },

            // upper right
            p_ur: Point { x: 5.0, y: 0.0 },

            // lower left
            p_ll: Point { x: 0.0, y: 5.0 },

            // lower right
            p_lr: Point { x: 5.0, y: 5.0 },
        };

        let _r2 = Rectangle {
            p_ul: Point { x: 0.0, y: 0.0 },

            // upper right
            p_ur: Point { x: 5.0, y: 0.0 },

            // lower left
            p_ll: Point { x: 0.0, y: 5.0 },

            // lower right
            p_lr: Point { x: 5.0, y: 5.0 },
        };
        //assert_eq!(collides_rectangles(&r1, &r2), true);
    }

    #[test]
    fn test_colliding_lines() {
        let mut p1 = Point { x: 1.0, y: 1.0 };
        let mut q1 = Point { x: 10.0, y: 1.0 };
        let mut p2 = Point { x: 1.0, y: 2.0 };
        let mut q2 = Point { x: 10.0, y: 2.0 };

        assert_eq!(line_intersect(&p1, &q1, &p2, &q2), false);

        p1 = Point { x: 10.0, y: 0.0 };
        q1 = Point { x: 0.0, y: 10.0 };
        p2 = Point { x: 0.0, y: 0.0 };
        q2 = Point { x: 10.0, y: 10.0 };

        assert_eq!(line_intersect(&p1, &q1, &p2, &q2), true);
    }
}
