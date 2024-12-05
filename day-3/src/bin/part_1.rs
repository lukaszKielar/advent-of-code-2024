fn main() {
    let input = include_str!("../../../inputs/day-3.txt");

    let res = day_3::part_1::process(input);

    println!("{:?}", res);
    assert_eq!(res, 160672468);
}
