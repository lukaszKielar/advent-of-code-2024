fn main() {
    let input = include_str!("../../../inputs/day-17.txt");

    let res = day_17::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}
