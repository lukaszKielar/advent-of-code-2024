fn main() {
    let input = include_str!("../../../inputs/day-18.txt");

    let res = day_18::part_1::process(input, 70, 1024);

    println!("res: {res}");
    assert_eq!(res, 322);
}
