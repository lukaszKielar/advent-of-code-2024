fn main() {
    let input = include_str!("../../../inputs/day-6.txt");

    let res = day_6::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 2188);
}
