fn main() {
    let input = include_str!("../../../inputs/day-N.txt");

    let res = day_N::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}
