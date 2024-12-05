fn main() {
    let input = include_str!("../../../inputs/day-5.txt");

    let res: usize = day_5::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 5509);
}
