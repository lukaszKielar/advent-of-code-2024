fn main() {
    let input = include_str!("../../../inputs/day-13.txt");

    let res = day_13::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}
