fn main() {
    let input = include_str!("../../../inputs/day-15.txt");

    let res = day_15::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 1349898);
}
