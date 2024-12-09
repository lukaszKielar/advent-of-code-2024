fn main() {
    let input = include_str!("../../../inputs/day-9.txt");

    let res = day_9::part_1::process(input);

    println!("res: {res}");
    assert_eq!(res, 6471961544878);
}
