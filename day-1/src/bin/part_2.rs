use std::collections::HashMap;

fn main() {
    // let input = "
    //     3   4
    //     4   3
    //     2   5
    //     1   3
    //     3   9
    //     3   3
    // ";
    let input = include_str!("../../../inputs/day-1.txt");

    let parsed_input = input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim().split_whitespace().collect::<Vec<_>>();
            (
                line[0].parse::<usize>().unwrap(),
                line[1].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let vec2_hashmap =
        parsed_input
            .iter()
            .map(|elem| elem.1)
            .fold(HashMap::<usize, usize>::new(), |mut m, x| {
                *m.entry(x).or_insert(0) += 1;
                m
            });

    let res: usize = parsed_input
        .iter()
        .map(|elem| {
            let occurences = vec2_hashmap.get(&elem.0).unwrap_or(&0);
            elem.0 * occurences
        })
        .sum();

    println!("{:?}", res);
}
