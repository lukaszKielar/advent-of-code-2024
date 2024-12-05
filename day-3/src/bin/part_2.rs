fn main() {
    let input = include_str!("../../../inputs/day-3.txt");

    let res = day_3::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 84893551);
}
