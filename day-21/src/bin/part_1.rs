fn main() {
    let input = include_str!("../../../inputs/day-21.txt");

    let res = day_21::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}
