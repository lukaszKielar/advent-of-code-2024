fn main() {
    let input = include_str!("../../../inputs/day-11.txt");

    let res = day_11::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 194557);
}
