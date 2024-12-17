fn main() {
    let input = include_str!("../../../inputs/day-19.txt");

    let res = day_19::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}
