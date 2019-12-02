use puzzlehandler;

fn main() {
    let input = puzzlehandler::ints_from_csv("input.csv".to_string());
    let input2 = puzzlehandler::ints_from_csv("input2.csv".to_string());
    println!("{:?}", input.unwrap());
    println!("{:?}", input2.unwrap());
}
