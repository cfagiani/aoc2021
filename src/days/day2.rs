use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day2 {}

impl Day for Day2 {
    /// Computes the product of depth and position after processing the directions in
    /// the input file
    fn part1(&self, input_root: &str) {
        let direction_tuples = get_data_from_file(input_root, "day2.txt", &parse_line);

        let mut x_pos = 0;
        let mut depth = 0;
        for t in direction_tuples {
            match t.0.as_str() {
                "up" => depth -= t.1,
                "down" => depth += t.1,
                "forward" => x_pos += t.1,
                _ => println!("unknown direction")
            }
        }
        println!("Product of position and depth is {}", x_pos * depth);
    }


    /// Computes product of depth and position, this time interpreting the directions to include the
    /// "aim" concept
    fn part2(&self, input_root: &str) {
        let direction_tuples = get_data_from_file(input_root, "day2.txt", &parse_line);

        let mut x_pos = 0;
        let mut depth = 0;
        let mut aim = 0;
        for t in direction_tuples {
            match t.0.as_str() {
                "up" => aim -= t.1,
                "down" => aim += t.1,
                "forward" => {
                    depth += aim * t.1;
                    x_pos += t.1;
                }
                _ => println!("unknown direction")
            }
        }
        println!("Product of position and depth is {}", x_pos * depth);
    }
}

fn parse_line<'a>(line: String) -> (String, i32) {
    let mut line_iter = line.split(" ");
    let dir = String::from(line_iter.next().unwrap());
    let num: i32 = line_iter.next().unwrap().parse().unwrap();
    (dir, num)
}
