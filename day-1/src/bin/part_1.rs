fn main() {
    let input = include_str!("../../../inputs/day-1.txt");

    let res = day_1::part_1::process(input);

    println!("{res}");
    assert_eq!(res, 1889772);
}
