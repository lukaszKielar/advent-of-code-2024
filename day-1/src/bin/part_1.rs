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

    let mut vec1 = parsed_input.iter().map(|elem| elem.0).collect::<Vec<_>>();
    vec1.sort_by(|a, b| a.cmp(b));

    let mut vec2 = parsed_input.iter().map(|elem| elem.1).collect::<Vec<_>>();
    vec2.sort_by(|a, b| a.cmp(b));

    let res: usize = std::iter::zip(vec1, vec2)
        .map(|elems| elems.0.abs_diff(elems.1))
        .sum();

    println!("{res}");
}
