fn main() {
    let input = include_str!("../../../inputs/day-5.txt");

    let res = day_5::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 4407);
}
