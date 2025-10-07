use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{parse_page_ordering_rules, sort_page_ordering_rules, Nums};

fn sort_line(line: Vec<usize>, page_ordering_rules: &HashMap<usize, Nums>) -> Vec<usize> {
    let nodes: HashSet<usize> = line.iter().copied().collect();
    if nodes.len() <= 1 {
        return line;
    }

    let positions: HashMap<usize, usize> = line
        .iter()
        .enumerate()
        .map(|(idx, &page)| (page, idx))
        .collect();

    let mut adj: HashMap<usize, Vec<usize>> = nodes
        .iter()
        .map(|&page| (page, Vec::new()))
        .collect();

    let mut indegree: HashMap<usize, usize> = nodes.iter().map(|&page| (page, 0)).collect();

    for (&before, nums) in page_ordering_rules {
        if !nodes.contains(&before) {
            continue;
        }

        if let Some(neighbors) = adj.get_mut(&before) {
            for &after in &nums.before {
                if !nodes.contains(&after) {
                    continue;
                }
                if neighbors.contains(&after) {
                    continue;
                }
                neighbors.push(after);
                *indegree.get_mut(&after).unwrap() += 1;
            }
        }
    }

    let mut heap: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    for (&page, &deg) in &indegree {
        if deg == 0 {
            let pos = positions.get(&page).copied().unwrap_or(usize::MAX);
            heap.push(Reverse((pos, page)));
        }
    }

    let mut result: Vec<usize> = Vec::with_capacity(nodes.len());

    while let Some(Reverse((_, page))) = heap.pop() {
        result.push(page);
        if let Some(neighbors) = adj.get(&page) {
            for &neighbor in neighbors {
                if let Some(entry) = indegree.get_mut(&neighbor) {
                    *entry -= 1;
                    if *entry == 0 {
                        let pos = positions.get(&neighbor).copied().unwrap_or(usize::MAX);
                        heap.push(Reverse((pos, neighbor)));
                    }
                }
            }
        }
    }

    if result.len() == nodes.len() {
        result
    } else {
        line
    }
}

pub fn process(input: &str) -> usize {
    let input = input.trim().split("\n\n").collect::<Vec<_>>();

    let page_ordering_rules = parse_page_ordering_rules(input[0]);
    let page_ordering_rules = sort_page_ordering_rules(page_ordering_rules);

    let page_numbers_on_each_update = input[1]
        .trim()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|elem| elem.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    page_numbers_on_each_update
        .into_iter()
        .filter(|line| {
            let mut status = false;

            for (index, elem) in line.iter().enumerate() {
                let before_elems = &line[..index];
                let after_elems = &line[index + 1..];

                let nums = Nums::new();
                let elem_nums = page_ordering_rules.get(elem).unwrap_or(&nums);

                if before_elems.iter().any(|e| elem_nums.before.contains(e))
                    || after_elems.iter().any(|e| elem_nums.after.contains(e))
                {
                    status = true;
                }
            }

            status
        })
        .map(|l| sort_line(l, &page_ordering_rules))
        .map(|l| l[l.len() / 2])
        .collect::<Vec<_>>()
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_process() {
        // given
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        // when
        let output = process(input);

        // then
        assert_eq!(output, 123)
    }

    #[rstest]
    #[case(
        vec![75, 97, 47, 61, 53],
        vec![97,75,47,61,53]
    )]
    #[case(
        vec![61, 13, 29],
        vec![61, 29, 13]
    )]
    #[case(
        vec![97, 13, 75, 29, 47],
        vec![97, 75, 47, 29, 13]
    )]
    fn test_sort_line(#[case] line: Vec<usize>, #[case] expected: Vec<usize>) {
        // given
        let pairs = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";
        let page_ordering_rules = parse_page_ordering_rules(pairs);
        let page_ordering_rules = sort_page_ordering_rules(page_ordering_rules);

        // when
        let output = sort_line(line, &page_ordering_rules);

        // then
        assert_eq!(output, expected)
    }
}
