fn main() {
    let input = include_str!("../../../inputs/day-2.txt");

    let res = day_2::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 663);
}
