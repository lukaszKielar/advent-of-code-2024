use crate::{parse_page_ordering_rules, sort_page_ordering_rules, Nums};

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
            let mut status = true;

            for (index, elem) in line.iter().enumerate() {
                let before_elems = &line[..index];

                let after_elems = &line[index + 1..];

                let nums = Nums::new();
                let elem_nums = page_ordering_rules.get(elem).unwrap_or(&nums);

                if before_elems.iter().any(|e| elem_nums.before.contains(e))
                    || after_elems.iter().any(|e| elem_nums.after.contains(e))
                {
                    status = false;
                }
            }
            status
        })
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
        assert_eq!(output, 143)
    }
}
