fn main() {
    let input = puzzlehandler::ints_from_csv("input.csv".to_string()).unwrap();
    puzzlehandler::resolve( Box::new( part1(input.clone()) ) );
    puzzlehandler::resolve( Box::new( part2(input.clone()) ) );
}

fn part1(input: Vec<i32>) -> i32 {
    let mut intcode = input.clone();
    intcode[1] = 12;
    intcode[2] = 2;
    return run_intcode(intcode);
}

fn part2(input: Vec<i32>) -> i32 {
    let mut noun = 0;
    while noun < 100 {
        let mut verb = 0;
        while verb < 100 {
            let mut intcode = input.clone();
            intcode[1] = noun;
            intcode[2] = verb;
            let result = run_intcode(intcode);
            if result == 19690720 {
                return format!("{}{}", noun, verb).parse::<i32>().unwrap();
            }
            verb += 1;
        }
        noun += 1;
    }
    println!("No solution found.");
    panic!();
}

fn run_intcode(input: Vec<i32>) -> i32 {
    let mut intcode = input.clone();
    let mut i = 0;
    while i < input.len() {
        match intcode[i] {
            1 => {
                let change_index = intcode[i+3] as usize;
                intcode[change_index] = intcode[intcode[i+1] as usize] + intcode[intcode[i+2] as usize];
            },
            2 => {
                let change_index = intcode[i+3] as usize;
                intcode[change_index] = intcode[intcode[i+1] as usize] * intcode[intcode[i+2] as usize];
            },
            99 => {
                break;
            },
            _ => {}
        }
        i += 4;
    }
    return intcode[0];
}

#[cfg(test)]
mod tests {
    #[test]
    fn compute_answer_works() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let result = super::run_intcode(input);
        assert_eq!(result, 3500);
    }
}