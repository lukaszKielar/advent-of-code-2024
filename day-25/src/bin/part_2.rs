fn main() {
    let input = include_str!("../../../inputs/day-25.txt");

    let res = day_25::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}