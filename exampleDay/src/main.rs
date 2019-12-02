use puzzlehandler;

fn main() {
    let input = puzzlehandler::ints_from_csv("input.csv".to_string());
    println!("{:?}", input.unwrap());
}
