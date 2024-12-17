fn main() {
    let input = include_str!("../../../inputs/day-20.txt");

    let res = day_20::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}
