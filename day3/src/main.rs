use puzzlehandler;
use std::cmp::{min, max};
use std::ops::Range;

fn main() {
    let input = puzzlehandler::string_matrix_from_csv("input.csv".to_string()).unwrap();
    puzzlehandler::resolve(Box::new(part1(input.clone())));
    puzzlehandler::resolve(Box::new(part2(input)));
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: i32,
    y: i32
}

fn part1(input: Vec<Vec<String>>) -> i32 {
    let (wire1_path, wire1_steps) = path_and_steps_from_input(&input[0]);
    // println!("{:?}", wire1_path);
    let (wire2_path, wire2_steps) = path_and_steps_from_input(&input[1]);
    // println!("{:?}", wire2_path);

    let (intersections, _) = intersections_and_total_steps_from_paths(wire1_path, wire2_path, wire1_steps, wire2_steps);
    // println!("{:?}", intersections);

    let mut distances: Vec<i32> = vec![];
    for intersection in intersections {
        distances.push(intersection.x.abs() + intersection.y.abs());
    }
    // println!("{:?}", distances);

    return *distances.iter().min().unwrap();
}

fn part2(input: Vec<Vec<String>>) -> i32 {
    let (wire1_path, wire1_steps) = path_and_steps_from_input(&input[0]);
    // println!("{:?}", wire1_path);
    // println!("{:?}", wire1_steps);
    let (wire2_path, wire2_steps) = path_and_steps_from_input(&input[1]);
    // println!("{:?}", wire2_path);
    // println!("{:?}", wire2_steps);

    let (_, total_steps) = intersections_and_total_steps_from_paths(wire1_path, wire2_path, wire1_steps, wire2_steps);
    // println!("{:?}", intersections);
    // println!("{:?}", total_steps);

    return *total_steps.iter().min().unwrap();
}

fn path_and_steps_from_input(input: &Vec<String>) -> (Vec<Coord>, Vec<i32>) {
    let mut coords: Vec<Coord> = vec![Coord{x: 0, y: 0}];
    let mut steps: Vec<i32> = vec![];
    let mut idx: usize = 1;

    for step in input {
        let mut step = step.clone();
        let direction = step.remove(0);
        let step: i32 = step.parse::<i32>().unwrap();
        steps.push(step);

        match direction {
            'U' => {
                coords.push(Coord{x: coords[idx - 1].x, y: coords[idx - 1].y + step});
            },
            'R' => {
                coords.push(Coord{x: coords[idx - 1].x + step, y: coords[idx - 1].y});
            },
            'D' => {
                coords.push(Coord{x: coords[idx - 1].x, y: coords[idx - 1].y - step});
            },
            'L' => {
                coords.push(Coord{x: coords[idx - 1].x - step, y: coords[idx - 1].y});
            },
            _ => {}
        }

        idx += 1;
    }

    return (coords, steps)
}

fn intersections_and_total_steps_from_paths(wire1_path: Vec<Coord>, wire2_path: Vec<Coord>, wire1_steps: Vec<i32>, wire2_steps: Vec<i32>) -> (Vec<Coord>, Vec<i32>) {
    let mut intersections: Vec<Coord> = vec![];
    let mut total_steps: Vec<i32> = vec![];

    // The general strategy is to loop over *line segments* in wire1 and then check if any
    // line segments in wire2 intersect.
    for (i, coord) in wire1_path.iter().enumerate() {
        // Ensure 2 is in range of wire1_path and ignore origin
        if i < wire1_path.len() - 1 && i != 0 {
            let x = coord.x;
            let y = coord.y;
            let next_x = wire1_path[i + 1].x;
            let next_y = wire1_path[i + 1].y;
            // Check each line segment in wire2 against the current wire1 line segment
            for (i2, coord2) in wire2_path.iter().enumerate() {
                // Ensure i2 is in range of wire2_path and ignore origin
                if i2 < wire2_path.len() - 1 && i2 != 0 {
                    let wire2_x = coord2.x;
                    let wire2_y = coord2.y;
                    let next_wire2_x = wire2_path[i2 + 1].x;
                    let next_wire2_y = wire2_path[i2 + 1].y;
                    // If line segment is horizontal
                    if y == next_y {
                        // Does the current horizontal wire1 segment have a Domain containing the x
                        // coordinate of the wire2 segment?
                        if (Range {start: min(x, next_x), end: max(x, next_x)} ).contains(&coord2.x) {
                            // Does the current vertical wire2 segment have a Aange containing the y coordinate
                            // of the current wire1 segment?
                            if (Range {start: min(wire2_y, next_wire2_y), end: max(wire2_y, next_wire2_y)} ).contains(&coord.y) {
                                // If so, it is an intersection!  We can get the precise location of the
                                // intersection by taking the x coordinate of the vertical line and the 
                                // y coordinate of the horizontal line.
                                intersections.push(Coord{x: coord2.x, y: y});
                                let summed_steps1: i32 = wire1_steps[0..i+1].iter().sum();
                                let summed_steps2: i32 = wire2_steps[0..i2+1].iter().sum();
                                total_steps.push((summed_steps1 - (next_x - coord2.x).abs()) + (summed_steps2 - (next_wire2_y - y).abs()));
                            }
                        }
                    // Else line segment is vertical
                    } else {
                        // The same comments above apply, except the below is reverse to account for when
                        // the wire1 segment is vertical, not horizontal like we assume above.
                        if (Range {start: min(y, next_y), end: max(y, next_y)} ).contains(&coord2.y) {
                            if (Range {start: min(wire2_x, next_wire2_x), end: max(wire2_x, next_wire2_x)} ).contains(&coord.x) {
                                intersections.push(Coord{x: x, y: coord2.y});
                                let summed_steps1: i32 = wire1_steps[0..i+1].iter().sum();
                                let summed_steps2: i32 = wire2_steps[0..i2+1].iter().sum();
                                total_steps.push((summed_steps1 - (next_y - coord2.y).abs()) + (summed_steps2 - (next_wire2_x - x).abs()));
                            }
                        }
                    }
                }
            }
        }
    }
    return (intersections, total_steps);
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_works() {
        let input = puzzlehandler::string_matrix_from_csv("test_input1.csv".to_string()).unwrap();
        let input2 = puzzlehandler::string_matrix_from_csv("test_input2.csv".to_string()).unwrap();
        let result = super::part1(input);
        let result2 = super::part1(input2);
        assert_eq!(result, 6);
        assert_eq!(result2, 135);
    }

    #[test]
    fn part2_works() {
        let input = puzzlehandler::string_matrix_from_csv("test_input1.csv".to_string()).unwrap();
        let input2 = puzzlehandler::string_matrix_from_csv("test_input2.csv".to_string()).unwrap();
        let input3 = puzzlehandler::string_matrix_from_csv("test_input3.csv".to_string()).unwrap();
        let result = super::part2(input);
        let result2 = super::part2(input2);
        let result3 = super::part2(input3);
        assert_eq!(result, 30);
        assert_eq!(result2, 410);
        assert_eq!(result3, 610);
    }
}