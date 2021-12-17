use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day17 {}

impl Day for Day17 {
    fn part1(&self, input_root: &str) {
        let bounding_box = get_data_from_file(input_root, "day17.txt", parse_target_area)[0];
        // if all we care about is height, we can ignore x since max height will be if land in target
        // while falling straight down. Fastest we can go and not overshoot the target is the
        // absolute value of the lowest target distance -1.
        let dy = i32::abs(bounding_box.1) - 1;
        // to get max height compute summation
        let summation = ((dy + 1) * dy) / 2;
        println!("Max height is {}", summation);
    }

    /// Tries every possible starting velocity that and counts which will let the probe reach
    /// the target area
    fn part2(&self, input_root: &str) {
        let bounding_box = get_data_from_file(input_root, "day17.txt", parse_target_area)[0];
        // we know we can't go any faster than this in the Y direction or else we'll overshoot
        let max_yy = i32::abs(bounding_box.1) - 1;
        let mut hits = 0;
        for vy in bounding_box.1..=max_yy {
            for vx in 1..=bounding_box.2 {
                let mut probe = Probe::new(0, 0, vx, vy);
                while probe.x < bounding_box.2 && probe.y > bounding_box.1 {
                    probe.advance();
                    if probe.is_in_box(bounding_box) {
                        hits += 1;
                        break;
                    }
                }
            }
        }
        println!("Hits {}", hits);
    }
}

fn parse_target_area(line: String) -> (i32, i32, i32, i32) {
    //target area: x=155..182, y=-117..-67
    let parts: Vec<String> = line
        .replace("target area: ", "")
        .split(", ")
        .map(String::from)
        .collect();
    let x_coords: Vec<i32> = parts[0]
        .replace("x=", "")
        .split("..")
        .map(|s| String::from(s).parse().unwrap())
        .collect();
    let y_coords: Vec<i32> = parts[1]
        .replace("y=", "")
        .split("..")
        .map(|s| String::from(s).parse().unwrap())
        .collect();
    return (x_coords[0], y_coords[0], x_coords[1], y_coords[1]);
}

struct Probe {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Probe {
    pub fn new(x: i32, y: i32, dx: i32, dy: i32) -> Probe {
        Probe { x, y, dx, dy }
    }

    pub fn advance(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        if self.dx > 0 {
            self.dx -= 1;
        } else if self.dx < 0 {
            self.dx += 1;
        }
        self.dy -= 1;
    }

    pub fn is_in_box(&self, bounding_box: (i32, i32, i32, i32)) -> bool {
        if self.x >= bounding_box.0 && self.x <= bounding_box.2 {
            return self.y >= bounding_box.1 && self.y <= bounding_box.3;
        }
        return false;
    }
}
