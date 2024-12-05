fn main() {
    let input = include_str!("../../../inputs/day-2.txt");

    let res = day_2::part_2::process(input);

    println!("{:?}", res);
    assert_eq!(res, 692);
}
