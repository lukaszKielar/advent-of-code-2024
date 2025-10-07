fn main() {
    let input = include_str!("../../../inputs/day-14.txt");

    let res = day_14::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 7037);
}
