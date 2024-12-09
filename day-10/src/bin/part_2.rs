fn main() {
    let input = include_str!("../../../inputs/day-10.txt");

    let res = day_10::part_2::process(input);

    println!("res: {res}");
    assert_eq!(res, 999);
}
