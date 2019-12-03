use puzzlehandler;
use std::cmp::{min, max};
use std::ops::Range;

fn main() {
    let input = puzzlehandler::string_matrix_from_csv("input.csv".to_string()).unwrap();
    // let input2 = puzzlehandler::ints_from_csv("input2.csv".to_string()).unwrap();
    puzzlehandler::resolve(Box::new(part1(input)));
    // puzzlehandler::resolve(Box::new(part2(input2)));
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: i32,
    y: i32
}

fn part1(input: Vec<Vec<String>>) -> i32 {
    let wire1_path = path_from_steps(&input[0]);
    // println!("{:?}", wire1_path);
    let wire2_path = path_from_steps(&input[1]);
    // println!("{:?}", wire2_path);

    let mut intersections: Vec<Coord> = vec![];
    for (i, coord) in wire1_path.iter().enumerate() {
        if i < wire1_path.len() - 1 && (coord.x != 0 && coord.y != 0) {
            let x = coord.x;
            let y = coord.y;
            let next_x = wire1_path[i + 1].x;
            let next_y = wire1_path[i + 1].y;
            // If line segment is horizontal
            if y == next_y {
                for (i2, coord2) in wire2_path.iter().enumerate() {
                    if i2 < wire2_path.len() - 1 {
                        if (Range {start: min(x, next_x), end: max(x, next_x)} ).contains(&coord2.x) {
                            let wire2_y = coord2.y;
                            let next_wire2_y = wire2_path[i2 + 1].y;
                            if (Range {start: min(wire2_y, next_wire2_y), end: max(wire2_y, next_wire2_y)} ).contains(&coord.y) {
                                intersections.push(Coord{x: coord2.x, y: y});
                            }
                        }
                    }
                }
            // Else line segment is vertical
            } else {
                for (i2, coord2) in wire2_path.iter().enumerate() {
                    if i2 < wire2_path.len() - 1 {
                        if (Range {start: min(y, next_y), end: max(y, next_y)} ).contains(&coord2.y) {
                            let wire2_x = coord2.x;
                            let next_wire2_x = wire2_path[i2 + 1].x;
                            if (Range {start: min(wire2_x, next_wire2_x), end: max(wire2_x, next_wire2_x)} ).contains(&coord.x) {
                                intersections.push(Coord{x: x, y: coord2.y});
                            }
                        }
                    }
                }
            }

            // if wire2_path.contains(coord) {
            //     intersections.push(*coord)
            // }
            // for (_i2, wire2_coord) in wire2_path.iter().enumerate() {
            //     if coord.x == wire2_coord.x && coord.y == wire2_coord.y {
            //         intersections.push(*coord)
            //     }
            // }
        }
    }
    // println!("{:?}", intersections);

    let mut distances: Vec<i32> = vec![];
    for intersection in intersections {
        distances.push(intersection.x.abs() + intersection.y.abs());
    }
    // println!("{:?}", distances);

    return *distances.iter().min().unwrap();
}

fn path_from_steps(steps: &Vec<String>) -> Vec<Coord> {
    let mut coords: Vec<Coord> = vec![Coord{x: 0, y: 0}];
    let mut idx: usize = 1;

    for step in steps {
        let mut step = step.clone();
        let direction = step.remove(0);
        let step: i32 = step.parse::<i32>().unwrap();

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

        // match direction {
        //     'U' => {
        //         for new_y in (coords[idx - 1].y)..(coords[idx - 1].y + step) {
        //             coords.push(Coord{x: coords[idx - 1].x, y: new_y});
        //         }
        //     },
        //     'R' => {
        //         for new_x in (coords[idx - 1].x)..(coords[idx - 1].x + step) {
        //             coords.push(Coord{x: new_x, y: coords[idx - 1].y});
        //         }
        //     },
        //     'D' => {
        //         for new_y in (coords[idx - 1].y - step)..(coords[idx - 1].y) {
        //             coords.push(Coord{x: coords[idx - 1].x, y: new_y});
        //         }
        //     },
        //     'L' => {
        //         for new_x in (coords[idx - 1].x - step)..(coords[idx - 1].x) {
        //             coords.push(Coord{x: new_x, y: coords[idx - 1].y});
        //         }
        //     },
        //     _ => {}
        // }

        idx += 1;
    }

    return coords
}

// fn part2(input: Vec<i32>) -> Vec<i32> {
//     return input;
// }

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

    // #[test]
    // fn part2_works() {
    //     let input = vec![5, 6, 7, 8, 9];
    //     let result = super::part2(input.clone());
    //     assert_eq!(result, input);
    // }
}