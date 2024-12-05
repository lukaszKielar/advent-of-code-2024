fn main() {
    let input = include_str!("../../../inputs/day-4.txt");

    let res = day_4::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 1877);
}
