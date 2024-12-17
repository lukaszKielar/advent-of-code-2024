fn main() {
    let input = include_str!("../../../inputs/day-22.txt");

    let res = day_22::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}
