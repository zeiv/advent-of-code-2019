use puzzlehandler;

fn main() {
    let input = puzzlehandler::ints_from_csv("input.csv".to_string()).unwrap();
    puzzlehandler::resolve(Box::new(part1(input.clone())));
    puzzlehandler::resolve(Box::new(part2(input.clone())));
}

fn part1(modules: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for module in modules {
        result += (module / 3) - 2;
    }
    return result;
}

fn part2(modules: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for module in modules {
        let mut fuel_for_module = (module / 3) - 2;
        let mut remaining_fuel_to_account_for = fuel_for_module;

        while remaining_fuel_to_account_for >= 0 {
            remaining_fuel_to_account_for = (remaining_fuel_to_account_for / 3) - 2;
            if remaining_fuel_to_account_for >= 0 {
                fuel_for_module += remaining_fuel_to_account_for;
            }
        }

        result += fuel_for_module;
    }
    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn compute_answer_works() {
        let modules = vec![12, 14, 1969, 100756];
        let result = super::part1(modules);
        assert_eq!(result, 34241);
    }

    #[test]
    fn compute_answer_part2_works() {
        let modules = vec![12, 14, 1969, 100756];
        let result = super::part2(modules);
        assert_eq!(result, 51316);
    }
}