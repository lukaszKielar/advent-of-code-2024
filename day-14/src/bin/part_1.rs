use day_14::{X, Y};

fn main() {
    let input = include_str!("../../../inputs/day-14.txt");

    let res = day_14::part_1::process(input, X, Y);

    println!("res: {res}");
    assert_eq!(res, 218965032);
}
