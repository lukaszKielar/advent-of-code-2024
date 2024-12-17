fn main() {
    let input = include_str!("../../../inputs/day-23.txt");

    let res = day_23::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}
