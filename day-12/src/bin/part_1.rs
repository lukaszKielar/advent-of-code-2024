fn main() {
    let input = include_str!("../../../inputs/day-12.txt");

    let res = day_12::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 1473276);
}
