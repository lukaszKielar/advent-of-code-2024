use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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

fn parse_pairs(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split("|")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn sort_pairs(input: Vec<Vec<usize>>) -> HashMap<usize, Nums> {
    let mut sorted_pairs = HashMap::new();

    for row in input {
        let before = row[0];
        let after = row[1];

        if !sorted_pairs.contains_key(&before) {
            let nums = Nums {
                before: vec![after],
                after: vec![],
            };
            sorted_pairs.insert(before, nums);
        } else {
            if let Some(nums) = sorted_pairs.get_mut(&before) {
                nums.insert_into_before(after);
            }
        }

        if !sorted_pairs.contains_key(&after) {
            let nums = Nums {
                before: vec![],
                after: vec![before],
            };
            sorted_pairs.insert(after, nums);
        } else {
            if let Some(nums) = sorted_pairs.get_mut(&after) {
                nums.insert_into_after(before);
            }
        }
    }

    sorted_pairs
}

fn main() {
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
    let input = include_str!("../../../inputs/day-5.txt");

    let input = input.trim().split("\n\n").collect::<Vec<_>>();

    let pairs = parse_pairs(input[0]);
    let pairs = sort_pairs(pairs);
    // println!("pairs {:?}", pairs);
    let page_numbers_on_each_update = input[1]
        .trim()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|elem| elem.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // println!(
    //     "page_numbers_on_each_update {:?}",
    //     page_numbers_on_each_update
    // );

    let res: usize = page_numbers_on_each_update
        .into_iter()
        .filter(|line| {
            // println!("---");
            // println!("line: {:?}", line);
            let mut status = true;

            for (index, elem) in line.iter().enumerate() {
                // println!("\telem: {elem}");
                let before_elems = &line[..index];
                // println!("\t\tbefore_elems: {:?}", before_elems);
                let after_elems = &line[index + 1..];
                // println!("\t\tafter_elems: {:?}", after_elems);

                let nums = Nums::new();
                let elem_nums = pairs.get(elem).unwrap_or(&nums);
                // println!("\t\telem_nums: {:?}", elem_nums);

                if before_elems.iter().any(|e| elem_nums.before.contains(e))
                    || after_elems.iter().any(|e| elem_nums.after.contains(e))
                {
                    status = false;
                }
                // if before_elems.iter().any(|e| elem_nums.after.contains(e))
                //     || after_elems.iter().any(|e| elem_nums.before.contains(e))
                // {
                //     return false;
                // }
            }
            // println!("status: {status}");
            status
        })
        .map(|l| l[l.len() / 2])
        .collect::<Vec<_>>()
        .into_iter()
        .sum();

    // assert_eq!(res, 143);
    println!("res: {res}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_parse_pairs() {
        // given
        let input = "47|53
97|13
97|61
97|47";

        // when
        let output = parse_pairs(input);

        // then
        assert_eq!(output, &[[47, 53], [97, 13], [97, 61], [97, 47]]);
    }

    #[rstest]
    fn test_sort_pairs() {
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
        let input = parse_pairs(input);

        // when
        let output = sort_pairs(input);

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
