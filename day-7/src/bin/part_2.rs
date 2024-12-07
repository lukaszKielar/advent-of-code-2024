fn main() {
    let input = include_str!("../../../inputs/day-7.txt");

    let res = day_7::process(input, &vec!["*", "+", "||"]);

    println!("res: {res}");
    assert_eq!(res, 333027885676693);
}
