fn main() {
    let input = include_str!("../../../inputs/day-9.txt");

    let res = day_9::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 6511178035564);
}
