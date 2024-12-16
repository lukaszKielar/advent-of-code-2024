fn main() {
    let input = include_str!("../../../inputs/day-16.txt");

    let res = day_16::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}
