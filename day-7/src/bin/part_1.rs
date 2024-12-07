fn main() {
    let input = include_str!("../../../inputs/day-7.txt");

    let res = day_7::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 6231007345478);
}
