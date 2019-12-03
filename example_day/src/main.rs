use puzzlehandler;

fn main() {
    let input = puzzlehandler::ints_from_csv("input.csv".to_string()).unwrap();
    let input2 = puzzlehandler::ints_from_csv("input2.csv".to_string()).unwrap();
    puzzlehandler::resolve(Box::new(part1(input)));
    puzzlehandler::resolve(Box::new(part2(input2)));
}

fn part1(input: Vec<i32>) -> Vec<i32> {
    return input;
}

fn part2(input: Vec<i32>) -> Vec<i32> {
    return input;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_works() {
        let input = vec![0, 1, 2, 3, 4];
        let result = super::part1(input.clone());
        assert_eq!(result, input);
    }

    #[test]
    fn part2_works() {
        let input = vec![5, 6, 7, 8, 9];
        let result = super::part2(input.clone());
        assert_eq!(result, input);
    }
}