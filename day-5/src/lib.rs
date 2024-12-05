use std::collections::HashMap;

pub mod part_1;
pub mod part_2;

#[derive(Debug, PartialEq, Clone)]
struct Nums {
    after: Vec<usize>,
    before: Vec<usize>,
}

impl Nums {
    fn new() -> Self {
        Self {
            after: vec![],
            before: vec![],
        }
    }

    fn insert_into_before(&mut self, num: usize) {
        if !self.before.contains(&num) {
            self.before.push(num);
        }
    }

    fn insert_into_after(&mut self, num: usize) {
        if !self.after.contains(&num) {
            self.after.push(num);
        }
    }
}

fn parse_page_ordering_rules(page_ordering_rules: &str) -> Vec<Vec<usize>> {
    page_ordering_rules
        .trim()
        .lines()
        .map(|line| {
            line.split("|")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn sort_page_ordering_rules(page_ordering_rules: Vec<Vec<usize>>) -> HashMap<usize, Nums> {
    let mut sorted_input = HashMap::new();

    for row in page_ordering_rules {
        let before = row[0];
        let after = row[1];

        if !sorted_input.contains_key(&before) {
            let nums = Nums {
                before: vec![after],
                after: vec![],
            };
            sorted_input.insert(before, nums);
        } else {
            if let Some(nums) = sorted_input.get_mut(&before) {
                nums.insert_into_before(after);
            }
        }

        if !sorted_input.contains_key(&after) {
            let nums = Nums {
                before: vec![],
                after: vec![before],
            };
            sorted_input.insert(after, nums);
        } else {
            if let Some(nums) = sorted_input.get_mut(&after) {
                nums.insert_into_after(before);
            }
        }
    }

    sorted_input
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_parse_page_ordering_rules() {
        // given
        let input = "47|53
97|13
97|61
97|47";

        // when
        let output = parse_page_ordering_rules(input);

        // then
        assert_eq!(output, &[[47, 53], [97, 13], [97, 61], [97, 47]]);
    }

    #[rstest]
    fn test_sort_page_ordering_rules() {
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
53|13";
        let page_ordering_rules = parse_page_ordering_rules(input);

        // when
        let output = sort_page_ordering_rules(page_ordering_rules);

        // then
        assert_eq!(
            output,
            HashMap::from([
                (
                    47,
                    Nums {
                        after: vec![97, 75],
                        before: vec![53, 13, 61, 29]
                    }
                ),
                (
                    53,
                    Nums {
                        after: vec![47, 75, 61, 97],
                        before: vec![29, 13]
                    }
                ),
                (
                    97,
                    Nums {
                        after: vec![],
                        before: vec![13, 61, 47, 29, 53, 75],
                    }
                ),
                (
                    13,
                    Nums {
                        after: vec![97, 61, 29, 47, 75, 53],
                        before: vec![],
                    }
                ),
                (
                    61,
                    Nums {
                        after: vec![97, 47, 75],
                        before: vec![13, 53, 29],
                    }
                ),
                (
                    75,
                    Nums {
                        after: vec![97],
                        before: vec![29, 53, 47, 61, 13],
                    }
                ),
                (
                    29,
                    Nums {
                        after: vec![75, 97, 53, 61, 47],
                        before: vec![13],
                    }
                ),
            ])
        )
    }
}
