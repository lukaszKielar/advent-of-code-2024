fn main() {
    let input = include_str!("../../../inputs/day-8.txt");

    let res = day_8::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 379);
}
