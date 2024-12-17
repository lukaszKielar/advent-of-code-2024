fn main() {
    let input = include_str!("../../../inputs/day-24.txt");

    let res = day_24::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}
